---
level: basic
---

# Readings with their time

`Timed<V, Tm = DeviceMicros<u32>> { t: Tm, value: V }` is a value with the time it was taken: a sample of a
property, an item of a stream (`Timed<Ampere>`). `t` is a [point](points.md) on a time scale, by default
`DeviceMicros<u32>`: µs since the device booted (origin `DeviceBoot`) in a `u32` that wraps every 71.6 minutes. It is
a `final_structure` struct of `t` and then `value`, the same bytes a `#[derive_shrink_wrap(final_structure)]` struct
of those two fields has, so no fields can be added to it later. `t` is the bytes of its storage alone, no prefix and
no number tag. In an API it is used with its arguments, `property!(ro current: Timed<Ampere>)`; its definition is in
the `ww_si` snapshot, so devices leave it out of their introspection data (see
[generic types in an API](../types.md#generic-types-in-an-api)).

```rust
use ww_si::{Ampere, Timed, origins::DeviceMicros};

let sample = Timed::new(DeviceMicros::from_raw(1_000_000u32), Ampere::milli(-1200i32));
assert_eq!(sample.t.raw(), 1_000_000);
```
