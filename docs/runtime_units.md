---
level: advanced
---

# Units known only at run time

Some units are not known when the API is written: an ADC input whose unit is set by configuration, a generic sensor
channel, a calibration that turns volts into bar. For those the unit goes on the wire as an `SIExpr`, and a value
with one is an `AnyQuantity { prefix, value, unit }`:

```rust
use ww_si::units::{self, AngularVelocity, Per, Radian, Second, Torque};
use ww_si::{AnyQuantity, Magnitude, MarkerId, Prefix, PrefixPolicy, Provenance, SIExpr, Unit, UnitDef, UnitNames, Volt};

// a ww_si unit, or a typed quantity turned into one: the unit's definition in full
let v = AnyQuantity::from(Volt::milli(3300u16));
assert_eq!(v.to_string(), "3300 mV");
assert_eq!(format!("{v:#}"), "3.3 V");
assert_eq!(v.to_quantity::<units::Volt>(), Some(Volt::milli(3300u16)));
let unit = SIExpr::of::<Per<Torque, AngularVelocity>>();
assert_eq!(unit.to_string(), "N·m/(rad/s)");

// the same value naming its unit by reference, for a reader that has a table of the units (the API's model)
let r = v.compact_in(&units::TABLE, &mut []).unwrap();
assert_eq!(r.unit, SIExpr::Ref(MarkerId { crate_name: "exact_units", name: "Volt" }));
assert_eq!(r.to_si_f64(&units::TABLE), Some(3.3));
assert_eq!(r.to_si_f64(&()), None); // a table that doesn't list it: no factor

// a unit no table lists, in full: its dimension and scale make it convertible to SI anyway
let rpm = SIExpr::Def(UnitDef {
    id: MarkerId { crate_name: "my_api", name: "Rpm" },
    names: UnitNames::new("rpm", PrefixPolicy::Decimal),
    dim: <Per<Radian, Second> as Unit>::DIM,
    quantity: None,
    // exactly π/30 rad/s: 2π per 60 s
    scale: Magnitude::new(1, 30, 0, 0, 1),
    provenance: Provenance::Defined,
});
let speed = AnyQuantity::new(Prefix::Kilo, 3u8, rpm);
assert_eq!(speed.to_string(), "3 krpm");
```

`SIExpr` is the same model as the markers, put on the wire. A leaf is a named unit: `Def(UnitDef)`, its whole
definition (identity, names, dimension, kind, exact scale, provenance), or `Ref(MarkerId)`, its identity alone, the
declaring crate and the marker's name. Above them are `Mul`, `Div`, `Pow` and `Sqrt` of other `SIExpr`s (through
`RefBox`, so decoding one reads only its top node). A marker's `UnitExpr` maps one to one onto an `SIExpr` with `Def`
leaves (`Unit::SI_EXPR`, `SIExpr::of::<U>()`; `SIExprOwned::from(&UnitExpr)` on a host), which reads alone anywhere.

A `Ref` reads through a `UnitTable`, which maps an identity to its definition: `units::TABLE` lists the units of
exact_units, a `UnitExpr` lists its own named units, a slice of tables chains them, and the API model a client has
lists every unit the API uses, the device's own and exact_units' alike. So the walks take the table:
`AnyQuantity::to_si_f64(&table)`, `display(&table)`, `SIExpr::dim`, `magnitude` and `same`. `Display` reads through
`units::TABLE`, and `to_quantity::<U>()` through `U`'s own definition. A `Ref` the table doesn't list is shown by its
name and has no dimension or factor, so a conversion over it refuses. Two crates' units of one name are two units:
`my_api::Rpm` and `fan_api::Rpm` are told apart by their crate.

`compact_in(&table, &mut arena)` (no allocation; a named unit needs no arena) and `AnyQuantityOwned::compact(&table)`
turn each `Def` the table lists into a `Ref`, refusing a `Def` whose identity the table lists with another
definition; `resolve` does the reverse. A value sent beside its API carries the compact form: `wire_weaver_client`
compacts what it writes into an `AnyQuantity` against the API's model, and a device can compact against the units
it uses. A tree that comes from another device is walked at most `MAX_UNIT_DEPTH` (32) levels deep: a deeper part has
no dimension and is shown as `…`, it can't overflow the stack.

`Dim` is on the wire as `DimWire`: the seven exponents of s, m, kg, A, K, mol, cd as a list of `INib32` halves, then
the distinguished bases as a sorted list of `BaseExp { base: MarkerId, exp: Exp }`, `final_structure`. `Dim::new` is
`const`; `DimOwned::checked_mul`, `checked_div`, `checked_pow` and `checked_sqrt` give `None` for a run-time result
whose exponents leave ±63½ or a root that isn't whole halves. A unit's names (`UnitNames`: symbol, ASCII form, long
names, aliases, UCUM code, prefixes, exact_units IO-9) travel with its definition. A unit's scale is an exact
`Magnitude`. These are the bytes (strings and their lengths are laid out as shrink_wrap does it, lengths at the end):

| Value                                                  | Bytes                                                  |
|--------------------------------------------------------|--------------------------------------------------------|
| `Dim` of `Volt` (s⁻³ m² kg A⁻¹)                        | `b1 81 43 00 00 07`                                    |
| `Dim` of `Per<Volt, Sqrt<Hertz>>` (s⁻⁵ᐟ² m² kg A⁻¹)    | `91 81 43 00 00 07`                                    |
| `Dim` of `AngularVelocity` (s⁻¹ rad)                   | `30 00 00 00`, `exact_units`, `Radian`, `46 39 17`, 24 B |
| `Dim` of `Per<Byte, Sample>` (bit Sample⁻¹)            | `00 00 00 00`, `exact_units`, `Bit`, `40`, `exact_units`, `Sample`, `30 63 93 39 27`, 41 B |
| `Magnitude` one                                        | `00 00`                                                |
| `Magnitude` of a degree (π/180)                        | `fe 80 fe 80 02 00 00 00 00 83 fc 00 00`               |
| `SIExpr::of::<units::Volt>()`, a `Def`                 | `00`, `exact_units`, `Volt`, `V`, `00`, `volt`, `volts`, `80`, `V`, `11 05 41 b1 81 43 00 00 00 00 00 10 70 a4 39 3d`, 45 B |
| the same compacted, a `Ref`                            | `10`, `exact_units`, `Volt`, `04 39`, 18 B             |
| `SIExpr::of::<Per<Torque, AngularVelocity>>()`         | 204 B; compacted, two `Ref`s: 52 B                     |
| `AnyQuantity::from(Volt::milli(3300u16))`              | `12 e4 0c`, the unit's 31 B, `7b`: 35 B                |
| the same compacted                                     | `12 e4 0c`, the unit's 18 B, `2a`: 22 B                |

The unit costs bytes on every value (22 bytes by reference, 35 in full, instead of 3 for 3300 mV), so a value whose
unit is known when the API is written stays a typed quantity, and a channel's unit is better sent once in its
description than with every reading: `ww_adc` has `ChannelInfo::unit: SIExpr<'i>` and plain numbers in its readings.
