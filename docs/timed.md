---
level: basic
---

# Readings with their time

A sensor reading is only half a fact without the moment it was taken. `Timed<V, Tm>` holds both: the value `V` and its time `t` of type `Tm`. A property can hold one, and a stream item usually is one. A power supply's current, for instance, is a `Timed<Ampere>`.

`t` is a [point](points.md) on a time scale. Its default type is `DeviceMicros<u32>`: microseconds since the device booted, kept in a `u32` that wraps every 71.6 minutes, as [Timestamps](timestamps.md) explains. The origin of the scale is `DeviceBoot`, the moment of the boot.

## Making a reading

[`book_time.rs:timed`] makes a reading of 25.5 °C taken one second after the boot, and checks its bytes:

```rust
--8<-- "ww_si/tests/book_time.rs:timed"
```

The reading is six bytes: four for the time and two for the temperature. The time carries no prefix and no number type, only the bytes of its storage, and the value follows it. In an API the type is written with its arguments, as `Timed<Ampere>` for a current.

[`book_time.rs:timed_show`] shows the parts of the same reading:

```rust
--8<-- "ww_si/tests/book_time.rs:timed_show"
```

```text
--8<-- "docs/generated/ww_si/timed.txt"
```

The origin decides the text of the time: `1 s since boot`. `Debug` adds the unit and the number type, `1000000 µs u32`, which is how the device stores it.

## What you can rely on

`Timed` is laid out as a struct of `t` and then `value`, with no room for more fields. It has the same bytes as a struct of those two fields declared with `final_structure`, so the layout can never change. Because `ww_si` defines `Timed`, a device leaves its definition out of its introspection data. [Generic types in an API](../types.md#generic-types-in-an-api) explains why.

In a `Series`, `t` is a column like any other. 32 `Timed<Volt>` samples at a 40 µs clock take 21 bytes together. [Series](../serdes/series.md) shows how.

## Where to go next

[Time points and the full time on the host](time_points.md) picks another time type, when 71.6 minutes between two items is not enough or a millisecond clock is enough, and shows the host side of a timed stream.
