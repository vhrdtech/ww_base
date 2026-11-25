---
level: basic
---

# Points and origins on the wire

A [`Fixed`](exact_units.md) is an amount: 5 K warmer, 3 ms later, 20° further round. A `Point` is a reading on a
scale that starts somewhere: 25 °C, 1013 hPa absolute, a heading. `exact_units` has one affine algebra for all of
them, `Point<O, Fixed<U, P, T, B>>`: the algebra, the origins the crate ships, declaring your own with `origin!`,
register formulas, conversions and angles that wrap are in the book, starting at
[Points and amounts](https://exact.vhrd.tech/points/points/).

On the wire a point is exactly its number: `DegC` (`Point<Celsius, Fixed<Kelvin, Centi, i16>>`) is an `i16`, the
origin is only in the type.
