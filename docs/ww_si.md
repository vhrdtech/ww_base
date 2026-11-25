---
level: basic
---

# ww_si

`ww_si` gives device APIs typed physical values: a quantity is an SI [prefix](#prefixes) plus a `ww_numeric`
`NumericValue` in whatever number type the device picks, with the unit in the type. The API says "this is a
voltage", each device decides whether that is a `u16` of mV or an `f32` of V, and the host reads them all.

```rust
use ww_si::{Ampere, Overflow, Prefix, Rounding, Volt, Watt};

let v = Volt::milli(3300u16);          // prefix Milli, value U16(3300)
let w = Volt::from_f32(3.3);           // prefix Unit, value F32(3.3)
assert_eq!(Volt::milli(1500u16), Volt::base(1.5f32)); // compares values, not representations
assert_eq!(v.to_f64(), 3.3);           // in V
assert_eq!(w.to_in::<i32>(Prefix::Milli, Rounding::NearestAway, Overflow::Checked), Some(3300));
assert_eq!(v.to_string(), "3300 mV");
let p: Watt = v * Ampere::milli(500u16);              // the compiler checks the units
```

`Volt::milli(3300u16)` is 3 bytes on the wire, `12 e4 0c`: the prefix and number type nibbles, then the `u16`.

When the API fixes the prefix and number type anyway (a timeout in ms as `u16`, a timestamp in µs as `u64`), a
[`Fixed`](exact_units.md) has them in the type and sends the number alone: `Micros<u32>` is the 4 bytes of the `u32`.
The units, dimensions, `Fixed` and `SIExpr` live in the [exact_units](exact_units.md) crate, which `ww_si` builds on
and re-exports (`ww_si::units`, `ww_si::Dim`, `ww_si::Fixed` are its items); `ww_si` adds the run-time quantities:
`Quantity`, `AnyQuantity`, `Timed` and the timestamp tools. [Which one to pick](#quantity-fixed-or-anyquantity), and
how they convert into each other, is below.

## One type for every quantity

Every quantity is the same generic struct, and every unit a zero-size marker type in `ww_si::units`:

```rust
pub struct Quantity<U: Unit, B: QuantityBounds = Unbounded> {
    pub prefix: Prefix,
    pub value: NumericValue,
    _unit: PhantomData<U>,      // no bits on the wire, no bytes in RAM
    _bounds: PhantomData<B>,    // the allowed range, see "Bounds" below
}

pub type Volt = Quantity<units::Volt>;
pub type Ampere = Quantity<units::Ampere>;
// .. one alias per unit
```

APIs and code write the aliases (`property!(ro vbus: Volt)`, `Timed<Ampere>`). Everything below is written once for
all quantities, so `Hertz::kilo(400u32)` works the same way as `Volt::milli(3300u16)`.

A marker knows its unit through the `Unit` trait: its dimension (`Dim`, exponents of s, m, kg, A, K, mol, cd and any
number of distinguished bases such as rad, bit and counted things, see [dimensions](#dimensions)), its name and symbol, and how a value becomes coherent SI (`Scale`: coherent, a power of ten, a factor, or a factor
and an offset for °C). `Volt::unit()` returns it; `ww_si::units::by_name("Volt")` finds it by the marker's name, which
is what a client that only sees introspection data uses.

## Quantity, Fixed or AnyQuantity

All three carry a value with its unit; they differ in what is fixed when the API is written and what travels with
every value:

| | Unit | Prefix and number type | On the wire | Pick it for |
|---|---|---|---|---|
| `Fixed<U, P, T, B>` | in the type | in the type | the number alone: `Micros<u32>` is 4 B | the API knows the scale and the storage: a timeout in ms as `u16`, a duty cycle in 1/65535, an ADC step |
| `Quantity<U, B>` | in the type | per value | a prefix and number type nibble, then the number: `Volt::milli(3300u16)` is 3 B | the API knows the unit, each device picks the scale and storage (`u16` of mV here, `f32` of V there) |
| `AnyQuantity` | per value | per value | the prefix, number and the unit: 3300 mV is 22 B with the unit by reference, 49 B in full | the unit is set at run time: configured sensor inputs, generic loggers |
| `Scaled<U, T>` | in the type | the scale per value, exact; the number type in the type | the number, then its `RunScale` in 9 B | the scale is read at run time but exact: a video stream's time base, a quantizer step ([below](#run-time-scales)) |

The units are one system in all three: the markers of `ww_si::units` (from [exact_units](exact_units.md)), with real
dimensions. A value only converts into another of the same dimension: `Byte`, `Sample` and `Ratio` are each a
dimension of their own (information, a counted thing, one) and never convert into each other, whatever their numbers.

Conversions, both ways and exact:

- `Quantity::try_from_fixed(f)` takes a `Fixed` as a `Quantity` with its prefix and number as they are: a named
  prefix where the scale has one (`Milli`, `Kibi`), else one of the [unnamed scales](#prefixes) `Pow10` (`Pow10<-4>`),
  `Pow2` (a Q format's 2⁻¹⁵) or `FullScale` (the 1/65535 of a `Duty16`). Any other scale (a BAM's π/2¹⁵, 1/5) is
  refused with `UnknownScale`: `AnyQuantity::from_fixed(f)` holds every `Fixed` exactly, the scale moved into its unit
  (`3·Bam16Rad·rad`). `Quantity::from_fixed(f)` is the infallible form for decimal prefixes.
- `q.to_fixed::<P, T, B>()` gives the `Fixed` back if the value is exact in scale `P`, fits `T` and is within `B`,
  else `None`: `Volt::milli(3300u16).to_fixed::<Milli, u16, Unbounded>()` is `Some`, to `Unit` it is `None`.
- `AnyQuantity::from(q)` puts the unit into the value; `AnyQuantity::to_quantity::<U>()` takes it out again when the
  unit is `U`'s ([below](#units-known-only-at-run-time)).

Adding, comparing and converting give the same result, or the same refusal, for a `Fixed` and the `Quantity` it
converts to: a property test runs both sides over every scale (`ww_si/tests/parity.rs`).
