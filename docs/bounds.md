---
level: basic
---

# Bounds

`B` is the range a value must be in, in the unit's coherent value, whatever prefix a value comes with: a supply rail
of 0 to 3.3 V is `Quantity<units::Volt, RangeDec<0, 33, -1>>`, and `3300 mV`, `3.3 V` and `3 V` are in it, `3301 mV`
is not. The markers are the ones `Fixed` uses (`Range`, `RangeU64`, `RangeDec`, `Unbounded`, the default).

```rust
use ww_si::{units, Prefix, Quantity, RangeDec};

type Rail = Quantity<units::Volt, RangeDec<0, 33, -1>>;
assert!(Rail::try_new(Prefix::Milli, 3300u16).is_ok());
assert!(Rail::try_new(Prefix::Milli, 3301u16).is_err());     // OutOfBounds
let v = Rail::try_new(Prefix::Unit, 3u8).unwrap().unbounded(); // a plain Volt again
```

A bounded quantity is made with `try_new`, or from a plain one with `bounded::<B>()`; the constructors without a check
(`milli`, `from_f32`, ..) are only on the unbounded `Quantity<U>`. Decoding checks the bounds too, also every item of
a `Series`: a device never sees an out-of-range value in a handler. The bounds are in the API model (`Unbounded`
too, named in every instance), so `ww` and the GUI show the limits of a `Quantity` the same way as of a `Fixed`.
Bounds cost no bytes on the wire: a bounded quantity has the bytes of the unbounded one.
