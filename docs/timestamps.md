---
level: basic
---

# Timestamps

A `Second` that marks when something happened (an ADC sample, the start and end of an I2C transaction) is the
device's time since its clock started. A device has one clock, so the timestamps of all its traits line up, and
`ww_timebase::Clock::now()` reads it in full. The device picks the prefix and the storage, and keeps them the same
for a given stream or method.

Fixed-width unsigned storage holds the time modulo `2^bits`, so it wraps:

| Storage          | Wraps after (µs) | Wraps after (ms) |
|------------------|------------------|------------------|
| `U16`            | 65.5 ms          | 65.5 s           |
| `U32`, `UVlq32`  | 71.6 min         | 49.7 days        |
| `U64`, `UVlq64`  | 584 000 years    | never            |

Timestamps are never signed or floats. On the host, `Unwrapper` (feature `host`) gets the full time back. It is
anchored on a full-width reading of the same clock, then takes each timestamp as the value with the same low bits
closest to the previous one:

```rust
use ww_si::{Second, Unwrapper};

let mut unwrapper = Unwrapper::new(Second::micro(5_000_000_123u64)).unwrap(); // Clock::now()
let full = unwrapper.unwrap(Second::micro(705_032_827u32)).unwrap();        // U32 µs from a stream
assert_eq!(full, Second::micro(5_000_000_123u64));
```

That works while consecutive timestamps are less than half a wrap period apart; re-anchor with `anchor(now)` after
a longer pause. Use one `Unwrapper` per stream. `U64` and signed values pass through unchanged. The anchor and the
timestamps can be in different prefixes, and the timestamps can change prefix and storage from one to the next.

Without a full reading of the clock, `Unwrapper::default()` takes the first timestamp as is (the earliest time it can
be) and counts on from there; `restart()` starts over, for a device whose clock restarted. This is what
`wire_weaver_client` does for every time a device sends ([timestamps](../api/stream_reliability.md#timestamps)).
