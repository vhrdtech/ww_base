---
level: advanced
---

# Unit algebra

`*` and `/` between quantities are checked by the compiler: only the products in the table below build, and the
result has the right type. `+` and `-` take two values of the same unit, a `Ratio` keeps the unit, and `* f32` /
`/ f32` scale a value.

```rust
use ww_si::{Ampere, AngularVelocity, Ratio, Torque, Volt, Watt};

let p: Watt = Volt::milli(3300u16) * Ampere::milli(500u16);   // 1.65 W
let p_mech: Watt = Torque::base(2u8) * AngularVelocity::base(100u8);
let i: Ampere = p / Volt::base(3.3f32);
let v_applied: Volt = Volt::base(48u8) * Ratio::percent(50u8); // 24 V
```

`Volt::base(1u8) * AngularVelocity::base(1u8)` and `Volt::base(1u8) + Ampere::base(1u8)` don't compile (the first
names the missing `UnitMul<AngularVelocity>` impl of `Volt`).

| `A * B = C` (also `B * A`, `C / A`, `C / B`) |                                           |
|-----------------------------------------------|-------------------------------------------|
| `Volt * Ampere = Watt`                        | `Torque * AngularVelocity = Watt`         |
| `Ampere * Ohm = Volt`                         | `Watt * Second = Joule`                   |
| `Ampere * Second = Coulomb`                   | `Farad * Volt = Coulomb`                  |
| `Volt * Second = Weber`                       | `Henry * Ampere = Weber`                  |
| `AngularVelocity * Second = Radian`           | `Speed * Second = Meter`                  |
| `Acceleration * Second = Speed`               | `Gram * Acceleration = Newton`            |
| `Newton * Meter = Joule`                      | `Hertz * Second = Ratio`                  |
| `HertzPerSecond * Second = Hertz`             |                                           |

Each line is checked against the SI exponents of its units when `ww_si` compiles, so the table can't be wrong; a
unit with an offset scale (°C) can't be in it. `Newton * Meter` is a `Joule` (work), a torque comes from `Torque`
values.

The numbers are computed in `f32` (in `f64` when an operand is stored as `F64`) through each unit's scale
(`Gram::kilo(2u8) * Acceleration::base(3u8)` is 6 N), and the result is stored as `F32` / `F64` without a prefix:
operators are for host and FPU math, the representation on the wire stays the device's choice through `quantize`.
They need the `f32` feature.

## Exact integer arithmetic

For values of the same unit in integer storage, timestamps and durations above all, there are exact methods that
keep integer storage and never round:

| Method                    | Result                                                                                |
|---------------------------|---------------------------------------------------------------------------------------|
| `a.checked_add(&b)`       | `a + b` in the finer prefix of the two and `a`'s number type                          |
| `a.checked_sub(&b)`       | `a - b`, the same way                                                                 |
| `a.checked_mul_int(n)`    | `a * n` (any integer up to the widest enabled type) in `a`'s prefix and number type   |
| `a.wrapping_add(&b)`      | `a + b` in `a`'s prefix and number type, wrapping at its width like a device clock    |
| `a.wrapping_sub(&b)`      | `a - b`, the same way                                                                 |
| `t0.steps(&dt)`           | `t0`, `t0 + dt`, `t0 + 2·dt`, .. without end, each as `wrapping_add` computes it      |

The `checked_*` ones give `None` when the result doesn't fit the number type; all of them give `None` for floats
(their operators are `+`, `-`, `*`) and the `wrapping_*` ones and `steps` when `b` is not a whole number of `a`'s
prefix (40 µs added to a time in ms), or when `b` in `a`'s prefix is beyond the widest enabled integer type.

```rust
use ww_si::Second;

assert_eq!(Second::milli(5u32).checked_add(&Second::micro(40u16)), Some(Second::micro(5040u32)));
assert_eq!(Second::micro(u32::MAX).checked_add(&Second::micro(1u8)), None);
assert_eq!(Second::micro(u32::MAX).wrapping_add(&Second::micro(2u8)), Some(Second::micro(1u32)));
// the times of 32 samples at 40 µs, the first one taken at t0
let t0 = Second::micro(4_294_967_000u32);
let times: Vec<Second> = t0.steps(&Second::micro(40u8)).unwrap().take(32).collect();
```

A batched stream's generated `<name>_push_many(state, t0, dt, values)` stamps a block of samples the same way, on
the [time point](time_points.md#which-time-point) of its `Timed` items.

## Arithmetic and conversions

A unit's quantity (STD-68, UNIT-8) keeps a voltage gain from converting into a duty cycle, a resistance from a
transimpedance, a width from a height and Bq from Hz, the compiler and the evolution checker alike: see the book's
[Quantities](https://exact.vhrd.tech/maths/quantities/) and, for the model, [Quantities in the
model](units_in_the_model.md#quantities). A ratio's quantity picks a gain's dB formula, 20·lg or 10·lg: [Decibels](https://exact.vhrd.tech/levels/decibels/).
Multiplying or dividing two `Fixed` values gives back the exact result unit and scale as a type, with checked,
saturating and wrapping forms for what doesn't fit: [Products and
requantization](https://exact.vhrd.tech/fixed_point/products/).
