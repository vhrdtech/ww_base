---
level: basic
---

# Temperatures and other points

A temperature of 25 °C and a temperature 5 K warmer are different kinds of number. The first is a place on a scale that starts somewhere, and the second is a distance. WireWeaver keeps them apart in the type. A `Fixed` is a distance, called an amount: 5 K warmer, 3 ms later, 20° further round. A `Point` is a reading on a scale: 25 °C, 1013 hPa absolute, a heading.

The distinction catches real mistakes. Adding two readings of 25 °C is meaningless, so the type refuses it. Adding an amount to a reading is fine, and so is subtracting two readings. This page uses the thermometer's temperature to show the rules.

## Points and amounts

The thermometer declares its temperature as `Point<Celsius, Fixed<Kelvin, Centi, i16>>`. The first argument is the origin, the place where the scale starts: 0 °C. The second is the `Fixed` that counts the distance from the origin, here in steps of 0.01 K stored in an `i16`. That range covers −327.68 °C to 327.67 °C, which is enough for a thermometer.

The figure marks two readings and the amount between them. Rules follow from it: two points give an amount, and a point plus an amount gives a point.

--8<-- "docs/assets/point_amount.svg"

*Two readings of the thermometer and the amount between them.*

[`book_units.rs:points`] does the same in code. `Temperature` is the alias declared in [Fixed numbers](exact_units.md):

```rust
--8<-- "ww_si/tests/book_units.rs:points"
```

```text
--8<-- "docs/generated/ww_si/points.txt"
```

The origin decides how a point is shown: `Display` writes degrees Celsius. The difference of two points is a plain `Fixed` in kelvin, shown as 300 cK, which means 300 hundredths of a kelvin, 3 K. The raw value −27315 is absolute zero, −273.15 °C.

## The same bytes as the number

A point costs nothing on the wire. It is exactly the number of its `Fixed`, and the origin exists only in the type. [`book_units.rs:point_bytes`] checks it:

```rust
--8<-- "ww_si/tests/book_units.rs:point_bytes"
```

A reading of 25.5 °C is 2550 steps, which are the two bytes of the `i16` 2550. The host knows from the API that these bytes count 0.01 K from 0 °C, so it shows `25.5 °C`.

## Other origins

Celsius is one origin among several. A timestamp is a point too: its origin is the moment the device booted, and [Readings with their time](timed.md) uses it. A moment in POSIX time has the origin `UnixEpoch`. You can declare an origin of your own, such as a register that reads 0 at 25 °C, with the `origin!` macro. The `exact_units` book covers the algebra of points, the origins it ships and your own, starting at [Points and amounts](https://exact.vhrd.tech/points/points/).

[Points and origins in the model and in tools](points_and_origins.md) shows what `ww` and Python make of a point.
