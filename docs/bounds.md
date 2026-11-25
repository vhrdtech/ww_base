---
level: basic
---

# Bounds

The [[bounds]] of a number are the smallest and the largest value its type accepts, both included. A thermometer's gain from 1 to 16 and a supply rail from 0 V to 3.3 V are two examples. The bounds are part of the type, so the type refuses a value outside them wherever the value is made or read. Your own code never has to check.

This page shows the bounds of a `Fixed` and of a `Quantity`, and the three places where they are checked. It continues the thermometer from [Fixed numbers](exact_units.md).

## Bounds of a Fixed

The fourth argument of `Fixed` is the bounds. `Range<1, 16>` on the thermometer's `Gain` accepts the stored values 1 to 16. A `Fixed` without a fourth argument is unbounded and accepts every value its storage holds.

The figure shows the gain's range. A write of 8 is inside it. A write of 0 or of 20 is outside, and the type refuses both.

--8<-- "docs/assets/bounds_line.svg"

*The bounds of `Gain` on a number line: 8 is accepted, 0 and 20 are refused.*

## Where a value is checked

A value meets the bounds in three places, and each one refuses a bad value in its own way.

1. **Building a value in your code.** `bounded` takes a value written in the source. A bad value fails the build in `const` code and panics anywhere else. `try_new` takes a value that arrives at run time, from a user or a file, and returns an error.
2. **Decoding bytes.** A device decodes a request before it calls your handler. A value outside the bounds is a decode error, so the handler never runs.
3. **A tool that knows only the introspection data.** The `ww` tool and the Python module read the bounds from the device's [[introspection]] data and refuse a bad value before they send anything.

[`book_units.rs:refused`] shows the first two:

```rust
--8<-- "ww_si/tests/book_units.rs:refused"
```

The first line of its output is the default gain, then come the results of `try_new` and of decoding the bytes of 8 and of 20:

```text
--8<-- "docs/generated/ww_si/refused.txt"
```

The [Properties](../basic/properties.md) chapter follows a write of 20 from a host to the device, and shows the third place in `ww` and Python.

## Bounds of a Quantity

A `Quantity` has the bounds in its type too, as `Quantity<Unit, Bounds>`. Its bounds are in the unit itself, whatever prefix a value comes with. `RangeDec<0, 33, -1>` stands for 0 to 3.3: its two numbers are written with the exponent as the third argument, so `33` with `-1` is 3.3.

[`book_units.rs:rail`] declares a supply rail from 0 V to 3.3 V and tries three values:

```rust
--8<-- "ww_si/tests/book_units.rs:rail"
```

```text
--8<-- "docs/generated/ww_si/rail.txt"
```

3300 mV and 3 V are inside the range, and 3301 mV is not. A bounded `Quantity` has no constructors without a check, such as `milli`: it is made with `try_new`.

## What bounds cost

Bounds cost no byte on the wire. A bounded value has the bytes of the unbounded one, and the device checks them while it decodes.

Bounds also appear in the tools. `ww introspect` prints `[1..=16]` after the gain's type, so a person at the command line sees the limits without reading the source.
