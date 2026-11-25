---
level: basic
---

# exact_units in an API

[`exact_units`](https://exact.vhrd.tech/) puts the unit, the scale and the storage of a number into its type,
`Fixed<U, P, T, B>`: a `T` in unit `U` and scale `P`, within the bounds `B`. On the wire and in memory it is exactly
the `T`: `shared_us: u64` becomes `shared: Fixed<Second, Micro, u64>` (or `Micros<u64>`) with the same bytes, and
introspection shows µs instead of a field name to trust. `Micros::<u32>::new(1500)` is 4 bytes on the wire,
`dc 05 00 00`, the bytes of the `u32` 1500.

The crate lives in its own repository, [romixlab/exact_units](https://github.com/romixlab/exact_units), with its
own book at [exact.vhrd.tech](https://exact.vhrd.tech/): it doesn't depend on WireWeaver, the wire format is its
optional `shrink_wrap` feature. [ww_si](ww_si.md) is built on it and re-exports everything, so `ww_si::Fixed` and
`exact_units::Fixed` are the same type; an API that already uses `ww_si` writes `ww_si::..`. For `Fixed` itself —
scales, storage, bounds, arithmetic, conversions, rounding, aliases, serde — see the book's
[first steps](https://exact.vhrd.tech/start/first_steps/) and [doing maths](https://exact.vhrd.tech/maths/arithmetic/).

## Fixed or Quantity

|                         | `Fixed<U, P, T, B>`                       | `ww_si::Quantity<U>`                        | `ww_si::AnyQuantity`            |
|-------------------------|-------------------------------------------|---------------------------------------------|---------------------------------|
| unit                    | in the type                               | in the type                                 | on the wire (an `SIExpr`)       |
| prefix and number type  | in the type, chosen by the API            | on the wire, chosen per device and value    | on the wire                     |
| bytes                   | the number alone                          | a prefix and type byte, then the number     | the number, prefix and the unit |
| on the device           | plain integer arithmetic                  | `ww_numeric::NumericValue`                  | the same                        |

Take `Fixed` when the API fixes the representation anyway (a timeout in ms as `u16`, a timestamp in µs as `u64`, a
buffer size in bytes): no byte for the prefix, and a `u64` field doesn't turn on `ww_numeric`'s 64-bit types for the
whole firmware. Take `Quantity` when each device should pick its own resolution (an ADC in mV on one board, in µV on
another). `Quantity::to_fixed` and `Quantity::from_fixed` convert between them.

## Aliases

`Secs`, `Millis`, `Micros`, `Nanos`, `Minutes`, `Hours`, `Days`, `Hz`, `Bytes`/`Kibibytes`/`Mebibytes`/`Gibibytes`,
`Count<Of, T>`, `Percent`/`Permille`/`Ppb`, `Duty16`, the Q formats and the binary angles are all `Fixed` under
another name. A moment in POSIX time is a `Point` of `origins::UnixEpoch`, aliased `UnixSecs`/`UnixMillis`/
`UnixMicros`/`UnixNanos`. See the book's [aliases table](https://exact.vhrd.tech/start/first_steps/#aliases-for-the-common-ones).

## In an API

`Fixed` goes into properties, method arguments, `Timed` values and `Series` like any type. The bounds may be left out
in Rust source; the walker fills in `Unbounded`, and the API model, snapshots, `ww`, Python and the scaffold always
show all four arguments:

```rust
use ww_api::prelude::*;
use ww_api::shrink_wrap::Series;
use ww_si::{Fixed, Micros, Range, Timed, prefix::{Milli, One}, units::{Byte, Second}};

#[ww_trait]
trait Device {
    property!(rw timeout: Fixed<Second, Milli, u16>);
    property!(rw window: Fixed<Second, Milli, u16, Range<1, 5000>>);
    fn sizes() -> Series<'i, Timed<Fixed<Byte, One, u32>>>;
    fn ticks() -> Series<'i, Timed<Micros<u32>>>;
}
```

A `Series` of `Fixed` values has the same columns and bytes as a `Series` of the plain numbers.
