---
level: advanced
---

# Run-time scales

A `Fixed`'s scale is a type; some scales are only known once the device runs, yet exact: a video stream's time base
(H.264's `num_units_in_tick ÷ time_scale`, an MP4 timescale), a JPEG quantization step, an HRD rate of
2^(6 + bit_rate_scale) bit/s. `ww_si::Scaled<U, T>` (exact_units' NUM-42) is a number `T` at such a scale, the unit
`U` still in the type, and `Ticks<T>` is `Scaled<Second, T>`:

```rust
use ww_si::{RunScale, Ticks};

// an MP4 track's timescale of 15360, read from the file
let base = RunScale::ratio(1, 15_360).unwrap();
let pts = Ticks::<u64>::new(15_360 * 10 + 512, base);
assert_eq!(pts.to_string(), "154112·1/15360 s");
```

A `RunScale` is `num / den · 2^pow2` with `num` and `den` odd and coprime `u32`s and `pow2` within ±127: one form per
value, so `RunScale::ratio(2002, 120_000)` is `RunScale::ratio(1001, 60_000)`, `(1001, 1875, -5)`. On the wire it is
`num` and `den` as `u32` and `pow2` as `i8` (`final_structure`, 9 bytes); a form that isn't the canonical one doesn't
decode, so a value has one encoding. A `Scaled` is its number, then its `RunScale`: the `Ticks<u64>` above is 17
bytes (`stdlib.si.ticks` in the [conformance corpus](../serdes/conformance.md)).

In the API model both are the derived structs, the unit of a `Scaled` its first type argument. `ww`, Python and the
dynamic client read and write a `Scaled` as the struct `{ value, scale: { num, den, pow2 } }` for now; showing it as a
quantity is CLT-71, and they don't run the canonical-form check, which is Rust code the model can't carry (SW-20): a
non-canonical scale they send is refused by the device. A scale that lives in another field or register (a SunSpec
`*_SF`, a BLE format descriptor's exponent) is STD-77.
