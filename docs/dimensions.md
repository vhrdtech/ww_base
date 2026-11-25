---
level: advanced
---

# Dimensions and your own units

## Dimensions

Besides the seven SI base dimensions, `Dim` has three kinds of its own, so the compiler and a client keep apart what
SI alone would call the same:

- **Angle (rad).** The plane angle is a base dimension: `AngularVelocity` is rad·s⁻¹ and not `Hertz` (s⁻¹),
  `Torque` is J/rad (kg·m²·s⁻²·rad⁻¹, written N·m), so `Torque * AngularVelocity = Watt` still balances and a torque
  isn't a joule. `Steradian` is rad², `Lumen` cd·rad².

  This departs from SI, where rad = 1, and has consequences: Hz and rad/s don't convert into each other by
  themselves, a code goes from one to the other with an explicit 2π (or through `Cycle` per second); a moment of
  inertia is kg·m²/rad² (`Per<Torque, AngularAcceleration>`), so torque over angular acceleration balances; an arc
  length r·θ carries rad (`Times<Meter, Radian>`), it isn't a plain length.
- **Information (bit).** `Bit` is the coherent unit; `Nibble` (4 bit), `Byte` (8 bit), `Halfword` (16 bit), `Word`
  (32 bit) and `Doubleword` (64 bit) are scales of it, like `Gram` to kilogram. `units::Nibble` is this unit, not
  `shrink_wrap::Nibble`, the 4-bit storage a `Fixed` may hold its number in. Bytes are never a `Ratio` or an angle,
  and `Per<Byte, Second>` is a data rate, not a frequency.
- **Counted things.** `Item`, `Sample`, `Frame`, `Packet`, `Line`, `Row`, `Record`, `Entry`, `Cycle`, `Block`,
  `Sector`, `Chunk`, `Char` and `Code` count things: each is a distinguished base of its own, so `Per<Sample, Second>`
  (Sample·s⁻¹) isn't `Hertz` and samples aren't packets. Bytes of a block are `Per<Byte, Block>`.
- **Distinguished bases.** Radian, bit, neper and every counted thing are bases beside SI's seven, each named by the
  crate and the marker that declare it (`MarkerId`): two crates' `Sample` are different bases, and a marker moved to
  another module of its crate stays the same base. A dimension holds any number of them (exact_units UNIT-10). An API
  crate declares its own with a `unit!` line ending in `base` (`pub struct Bucket = "bucket", base;`), or a scaled
  unit of another one's base with `base(Other)` (the decibel is `base(Neper)`); `ww_si::base!(Bucket)` is its
  `MarkerId`. `rad` and `bit` are written by those names, any other base by its marker's name. On the wire the bases
  follow the exponents as a list sorted by crate and name; a list out of order, with a base twice or with an exponent
  of 0 is refused on decode, so every dimension has exactly one encoding. `Dim::strip(MarkerId::RADIAN)` drops a base
  on purpose (the joule of a torque, for v = ωr), `true_part()` drops all of them.
- **Square roots.** A noise density in V/√Hz has half exponents. Every exponent of a `Dim` counts halves (`Exp`,
  ±63½), so V/√Hz is kg·m²·s⁻⁵ᐟ²·A⁻¹. `DimOwned::checked_sqrt` refuses an odd count of halves: a fourth root isn't a
  dimension.

```rust
use ww_si::units::{Byte, Per, Sample, Sqrt};
use ww_si::Unit;

let per_sample = <Per<Byte, Sample> as Unit>::DIM;
assert_eq!(per_sample.to_string(), "bit·Sample⁻¹");
let bytes = per_sample.checked_mul(&<Sample as Unit>::DIM).unwrap();
assert!(bytes.same(<Byte as Unit>::DIM));

let density = <Per<ww_si::units::Volt, Sqrt<ww_si::units::Hertz>> as Unit>::DIM;
assert_eq!(density.to_string(), "kg·m²·s⁻⁵ᐟ²·A⁻¹");

let torque = <ww_si::units::Torque as Unit>::DIM;
assert_eq!(torque.strip(ww_si::MarkerId::RADIAN).to_string(), "kg·m²·s⁻²");
```

## Compound units and your own

A unit outside the list is written with `Per<A, B>` and `Times<A, B>` of markers. They are not normalized:
`Per<Meter, Second>` is not `Speed`, it is shown as `m/s` and has the same dimension.

```rust
use ww_si::Quantity;
use ww_si::units::{AngularVelocity, Per, Times, Torque};

/// Torque = b·ω.
let b = Quantity::<Per<Torque, AngularVelocity>>::from_f32(1.2e-4);
assert_eq!(b.to_string(), "0.00012 N·m/(rad/s)");
/// Propeller: torque = k·ω·|ω|.
let k = Quantity::<Per<Torque, Times<AngularVelocity, AngularVelocity>>>::base(2u8);
assert_eq!(k.to_string(), "2 N·m/((rad/s)·(rad/s))");
```

An API crate can declare its own marker: a field-less struct deriving `Copy, Clone, Debug, Default` and an `impl
Unit` with its dimension and `UnitExpr` (see the `Unit` rustdoc, a revolutions-per-minute example). It works with
every method above; it has no products, and clients that don't know it show it by name. Its `UnitExpr` can be a
power too, `UnitExpr::Pow(&METER, 2)` is shown as `m²`; a marker whose `UnitExpr` is not a named one writes its
[run-time form](runtime_units.md) `SI_EXPR` as well.
