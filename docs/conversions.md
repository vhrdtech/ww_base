---
level: advanced
---

# Converting and comparing quantities

- `to_f64()` / `to_f32()`: the value in the unit itself (V, A, Hz, s, °, g);
- `to_f64_in(prefix)`: in another prefix, `Volt::from_f32(3.3).to_f64_in(Prefix::Milli)` is about 3300.0;
- `to_in::<T>(prefix, Rounding, Overflow)`: the value in units of `prefix` as the number type `T` (any primitive
  integer, `f32` / `f64` with those features). The call names what happens to a value that isn't a whole number of
  `prefix` (`Rounding`: `Truncate`, `Floor`, `Ceil`, `NearestEven`, `NearestAway`) and to one that doesn't fit `T`
  (`Overflow`: `Checked` gives `None`, `Saturate` the nearest end, `Wrap` modulo 2^bits, what a wrapping 32-bit
  counter of that unit shows, to compare a timestamp with a device clock). Exact for integer storage whatever the two
  scales are (SI, binary, `Pow10`, `Pow2`, `FullScale`); a float is first scaled to the float nearest the exact
  product, then rounded exactly. `Second::milli(5u16).to_in::<u32>(Prefix::Micro, Rounding::Truncate,
  Overflow::Checked)` is `Some(5000)`, `Volt::milli(3999u16).to_in::<u32>(Prefix::Unit, Rounding::NearestEven,
  Overflow::Checked)` is `Some(4)`;
- `to_si_f64()`: in coherent SI through the unit's scale: 90 ° is 1.5708 (rad), 20 °C is 293.15 (K), 5 g is 0.005 (kg);
- `rescaled(prefix, Rounding, Overflow)`: the same value in another prefix and the same number type, rounded and
  settled as `to_in` does, `None` also for a result outside the bounds `B`;
- `rescaled_exact(prefix)`: the same, only if that is exact (`Volt::milli(3000u16)` to `Unit` is 3 V; 3300 mV is
  `None`, and so is 3300 mV to µV, which doesn't fit a `U16`), for autoranging before sending;
- `same_repr(&other)`: same prefix and the same number in the same type, a cheap "did it change since I sent it".

`PartialEq` and `PartialOrd` compare values across prefixes and number types: `Volt::milli(1000u16) == Volt::base(1u8)`.
Two integers are compared exactly, even 60 orders of magnitude apart, and so are an `f32` and an integer (the float's
own value); two `f32`s at different prefixes compare by the scaled float, an `F64` (hosts) through `f64`.

On a small MCU the conversions, comparisons and bounds run on `exact_units`' integer kernel (`PrefixRatio`,
`Prefix::rescale`, `Prefix::cmp_scaled`) at the width of the enabled number types (`ww_numeric::Integer`, sign and
magnitude): with types up to 32 bits no `f32`, `f64` or 128-bit code is linked, and none for floats at all unless the
API turns on `f32` (`f64` is for hosts). The same prefix takes a shortcut on the stored numbers. The no-float guard
(`just no-float-guard`, part of `just test`) builds a firmware calling each of these operations for Cortex-M0 and
M4F and fails on any such symbol. Against the `i128` / `f64` paths this replaced, Portwright (Cortex-M0+, opt-level
`z`) is 14.6 KB smaller and rePort (Cortex-M33) 9.7 KB; a `FullScale` or binary-prefix conversion takes about a
tenth of the instructions it did, one between two SI prefixes about as many (`qemu-si-convert`).

`Display` writes the number as stored with its prefix, `{:#}` in the unit itself; `Debug` adds the unit's name and
the number type, `defmt::Format` is like `Display`:

| Value                          | `{}`           | `{:#}`    | `{:?}`              |
|--------------------------------|----------------|-----------|---------------------|
| `Volt::milli(3300u16)`         | `3300 mV`      | `3.3 V`   | `Volt(3300 mV U16)` |
| `Torque::milli(250i32)`        | `250 mN·m`     | `0.25 N·m` |                    |
| `Gram::kilo(5u8)`              | `5 kg`         | `5000 g`  |                     |
| `Ratio::percent(50u8)`         | `50 %`         | `0.5`     |                     |

`Baud` is a symbol rate, the baud rate of a UART or a protocol decoder, kept apart from `Hertz` so a baud rate is
never taken for a clock frequency (and a tool shows `115200 Bd`, not `115.2 kHz`). `Torque` (N·m) has the dimension
of `Joule` but is kept apart the same way. `Ratio` is a plain fraction (0.5 is half; `percent` is `Centi`), for a duty
cycle, a probability or a scale factor; `DeciBel` is the logarithmic one.
