---
level: advanced
---

# How a Quantity is built

Every quantity is the same generic struct, and every unit a zero-size marker type in `ww_si::units`:

```rust
pub struct Quantity<U: Unit, B: QuantityBounds = Unbounded> {
    pub prefix: Prefix,
    pub value: NumericValue,
    _unit: PhantomData<U>,      // no bits on the wire, no bytes in RAM
    _bounds: PhantomData<B>,    // the allowed range, see [Bounds](bounds.md)
}

pub type Volt = Quantity<units::Volt>;
pub type Ampere = Quantity<units::Ampere>;
// .. one alias per unit
```

APIs and code write the aliases (`property!(ro vbus: Volt)`, `Timed<Ampere>`). Everything below is written once for
all quantities, so `Hertz::kilo(400u32)` works the same way as `Volt::milli(3300u16)`.

A marker knows its unit through the `Unit` trait: its dimension (`Dim`, exponents of s, m, kg, A, K, mol, cd and any
number of distinguished bases such as rad, bit and counted things, see [dimensions](dimensions.md)), its name and symbol, and how a value becomes coherent SI (`Scale`: coherent, a power of ten, a factor, or a factor
and an offset for °C). `Volt::unit()` returns it; `ww_si::units::by_name("Volt")` finds it by the marker's name, which
is what a client that only sees introspection data uses.


## Converting between Fixed, Quantity and AnyQuantity

The units are one system in all three: the markers of `ww_si::units` (from [exact_units](exact_units.md)), with real
dimensions. A value only converts into another of the same dimension: `Byte`, `Sample` and `Ratio` are each a
dimension of their own (information, a counted thing, one) and never convert into each other, whatever their numbers.

Conversions, both ways and exact:

- `Quantity::try_from_fixed(f)` takes a `Fixed` as a `Quantity` with its prefix and number as they are: a named
  prefix where the scale has one (`Milli`, `Kibi`), else one of the [unnamed scales](prefixes.md) `Pow10` (`Pow10<-4>`),
  `Pow2` (a Q format's 2⁻¹⁵) or `FullScale` (the 1/65535 of a `Duty16`). Any other scale (a BAM's π/2¹⁵, 1/5) is
  refused with `UnknownScale`: `AnyQuantity::from_fixed(f)` holds every `Fixed` exactly, the scale moved into its unit
  (`3·Bam16Rad·rad`). `Quantity::from_fixed(f)` is the infallible form for decimal prefixes.
- `q.to_fixed::<P, T, B>()` gives the `Fixed` back if the value is exact in scale `P`, fits `T` and is within `B`,
  else `None`: `Volt::milli(3300u16).to_fixed::<Milli, u16, Unbounded>()` is `Some`, to `Unit` it is `None`.
- `AnyQuantity::from(q)` puts the unit into the value; `AnyQuantity::to_quantity::<U>()` takes it out again when the
  unit is `U`'s (see [Units known only at run time](runtime_units.md)).

Adding, comparing and converting give the same result, or the same refusal, for a `Fixed` and the `Quantity` it
converts to: a property test runs both sides over every scale (`ww_si/tests/parity.rs`).

