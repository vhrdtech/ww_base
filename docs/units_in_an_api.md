---
level: advanced
---

# Units in properties, methods and series

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

