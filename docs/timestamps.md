---
level: basic
---

# Timestamps

A `Second` that marks when something happened is a timestamp: an ADC sample, the start of an I2C transaction, the moment a button was pressed. Its value is the device's time since its clock started. A device has one clock, so the timestamps of all its traits line up with each other. `ww_timebase::Clock::now()` reads the clock in full, as [ww_timebase](ww_timebase.md) describes.

The device picks the prefix and the number type of its timestamps and keeps them the same for a given stream or method. Small types keep the messages short, and they have a cost: the clock wraps. This page shows when it wraps and how the host gets the full time back.

## A clock that wraps

Unsigned storage of a fixed width holds the time modulo 2^bits, so the number counts up to its largest value and starts again at 0. A 32-bit microsecond clock does this every 71.6 minutes. The table below lists the periods of the usual choices. Timestamps are never signed numbers or floats.

```text
--8<-- "docs/generated/ww_si/wrap_periods.txt"
```

The 64-bit rows wrap after hundreds of thousands of years or more, which is never in practice. The figure shows a 32-bit clock wrapping twice. The device sends only the low bits, so the host sees a number that falls back to 0.

--8<-- "docs/assets/clock_wrap.svg"

*A 32-bit clock wraps every 71.6 minutes. The host keeps its own line of full times, which never falls back.*

On the device, arithmetic on a timestamp wraps like the clock does. [`book_time.rs:wrap`] adds two ticks to the last value of a 32-bit clock:

```rust
--8<-- "ww_si/tests/book_time.rs:wrap"
```

The result is 1, not 4294967297, which is what a device clock shows.

## Getting the full time back on the host

The host turns each wrapped timestamp back into the full time with an [`Unwrapper`]. It starts from one full reading of the same clock, the anchor. It then takes each timestamp as the full time whose low bits match and which is closest to the previous one. [`book_time.rs:unwrapper`] anchors on 5000.000123 s and feeds it one timestamp from a stream:

```rust
--8<-- "ww_si/tests/book_time.rs:unwrapper"
```

The unwrapper finds the full time, 5000.000123 s, because it lies closest to the anchor among all the times that end in the same 32 bits.

This works while two consecutive timestamps are less than half a wrap period apart. After a longer pause, anchor again with `anchor(now)`. Use one `Unwrapper` per stream. A 64-bit timestamp and a signed one pass through unchanged. The anchor and the timestamps can use different prefixes, and the timestamps can change prefix and storage from one item to the next.

Without a full reading of the clock, `Unwrapper::default()` takes the first timestamp as it is, which is the earliest time it can be, and counts on from there. `restart()` starts over, for a device whose clock restarted. This is what `wire_weaver_client` does for every time a device sends, as [Timestamps](../api/stream_reliability.md#timestamps) explains.

## Where to go next

[Readings with their time](timed.md) puts a timestamp next to a value, which is what a property or a stream item usually needs.
