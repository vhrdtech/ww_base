---
level: basic
---

# Numbers with units

A number on the wire is only bytes. A field named `timeout_ms` promises milliseconds, but nothing checks the promise, and a host that reads the number as seconds gets no warning. WireWeaver puts the meaning of a number into its type instead: the unit, the step, the storage and the allowed range. The type travels in the [[introspection]] data of the device, so tools and other programs read the meaning you wrote.

This page names the two types you use most and shows how to pick between them. The pages after it teach each one: [Fixed numbers](exact_units.md), [Bounds](bounds.md), [Quantities with a prefix](quantity.md), then [Temperatures and other points](points.md), [Timestamps](timestamps.md) and [Readings with their time](timed.md). They use the thermometer of the [Properties](../basic/properties.md) chapter: its sampling rate, its gain and its temperature.

## Fixed or Quantity

Both types carry the unit in the type. They differ in what else the type fixes, and so in what each value carries on the wire:

--8<-- "docs/assets/numbers_pick.svg"

*A timeout of 1500 µs as a `Fixed` and as a `Quantity`: the `Fixed` sends the number, the `Quantity` also sends its prefix and number type.*

A `Fixed` fixes everything in the type: the unit, the step, the storage and the bounds. A `Quantity` fixes only the unit, and each value says which prefix and which number type it uses. A third type, `AnyQuantity`, also carries its unit on the wire, for units that are known only while the device runs. It is the subject of [Units known only at run time](runtime_units.md).

| Type | Unit | Step and storage | Pick it when |
|---|---|---|---|
| `Fixed` | in the type | in the type | the API knows them: a timeout in milliseconds as a `u16`, a gain from 1 to 16 |
| `Quantity` | in the type | in each value | each device should choose them: an ADC reading in millivolts on one board, in volts as an `f32` on another |

The thermometer's API knows exactly what it needs, so it uses `Fixed` for its sampling rate, its gain and its temperature. A voltmeter that runs on several boards is the case for `Quantity`. When in doubt, take `Fixed`: its value is the smallest on the wire, and a firmware that uses it compiles no more number types than it needs.

The next sections show the two values as bytes, with the code from [`book_units.rs:kinds`].

```rust
--8<-- "ww_si/tests/book_units.rs:kinds"
```

Both assertions run when the crate's tests run, so the bytes on this page are the bytes the code produces.

## Where the types live

The units, the `Fixed` type and the markers such as `Second` and `Kelvin` live in the `exact_units` crate. It does not depend on WireWeaver, and its wire format is an optional feature. The `ww_si` crate builds on it and re-exports everything, so `ww_si::Fixed` and `exact_units::Fixed` are the same type. An API that already depends on `ww_si` writes `ww_si::Fixed`.

`ww_si` adds what needs a run-time value: [`Quantity`], `AnyQuantity`, [`Timed`] and the timestamp tools. Its cargo features choose which unit groups and number types a firmware compiles, as [Features of ww_si](ww_si_features.md) explains. No feature changes the bytes on the wire.

## Where to go next

- [Fixed numbers](exact_units.md) declares the thermometer's types and shows their bytes.
- [Units in ww, Python and dynamic clients](units_in_tools.md) shows what the tools make of these types.
- [How a Quantity is built](quantity_type.md) and [Converting and comparing quantities](conversions.md) go deeper into `Quantity`.
