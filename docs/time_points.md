---
level: advanced
---

# Time points and the full time on the host

## Which time point

`Tm` is any `TimePoint`: a `Point<O, Fixed<Second, P, T>>` of a linear origin in seconds, in integer steps `P`
stored in an integer `T`, without bounds. The step and the storage are fixed by the type, per stream. Pick the one
that holds the longest silence between two items of a stream, at the device's resolution:

| `Tm`                                               | Wraps after | Longest gap the host bridges |
|----------------------------------------------------|-------------|------------------------------|
| `DeviceMillis<u16>`                                | 65.5 s      | 32.7 s                       |
| `DeviceMicros<u32>` (default)                      | 71.6 min    | 35.8 min                     |
| `Point<DeviceBoot, Fixed<Second, Pow2<-15>, u32>>` | 36.4 h      | 18.2 h                       |
| `DeviceMillis<u32>`                                | 49.7 days   | 24.8 days                    |
| `DeviceMicros<u64>`                                | never       | no unwrapping needed         |

A device that keeps a known time scale sends it the same way: `UnixMicros<u64>` for one that keeps UTC. A device
whose resolution changes at run time picks one `Tm` fine and wide enough for all of it; a fixed `u32` column in a
`Series` costs less than a per-item prefix and number tag did. A batched stream picks its `Tm` with
[`batched(time = ..)`](../api/batching.md).

On the device only the wrapping operations of `TimePoint` are used: `t.wrapping_add(dt)` and `t0.steps(dt)` (`t0`,
`t0 + dt`, `t0 + 2·dt`, .., wrapping like the clock), with `dt` a `TimeStep<Tm>` (`Fixed<Second, Micro, u32>` for
the default). `TimePoint::low(full)` takes the low bits of a full reading of the clock. `+` of a `u32` point panics
in a debug build once the clock wraps.

```rust
use ww_si::{TimePoint, TimeStep, origins::DeviceMicros};

let t0 = DeviceMicros::from_raw(4_294_967_000u32);
let dt = TimeStep::<DeviceMicros<u32>>::new(40);
// the times of 32 samples at 40 µs, the first one taken at t0, wrapping past 2^32 µs
let times: Vec<DeviceMicros<u32>> = t0.steps(dt).take(32).collect();
assert_eq!(times[8].raw(), 24);
```

## The full time on the host

`TimeUnwrapper<Tm>` gets the full point back, `TimePoint::Full` (`DeviceMicros<u64>` for the default): each point is
taken as the one closest to the previous with the same low bits. It is anchored on a full reading of the same clock
(`new(now)`, `anchor(now)`), or without one (`default()`) takes the first point as is and counts on from there;
`restart()` starts over for a device whose clock restarted. Use one per stream. The typed client does this for a
timed batched stream: its items carry `t: Option<DeviceMicros<u64>>`.

```rust
use ww_si::{TimeUnwrapper, origins::DeviceMicros};

let now = DeviceMicros::<u64>::from_raw(5_000_000_123); // the device's clock read in full
let mut unwrapper = TimeUnwrapper::<DeviceMicros<u32>>::new(now);
let full = unwrapper.unwrap(DeviceMicros::from_raw(705_032_827)).unwrap(); // u32 µs from a stream
assert_eq!(full, now);
```

A full `DeviceBoot` point is placed on another scale with `t.against(boot)`, `boot` being where the device booted on
that scale (a `UnixMicros<u64>`): exact, an offset only, right for a device whose clock was set or steered. A
device clock that runs at its own rate goes through a clock sync fit first.

In a `Series` (see [series](../serdes/series.md)) `t` is a column like any other: 32 `Timed<Volt>` samples at a
40 µs clock are 21 bytes.
