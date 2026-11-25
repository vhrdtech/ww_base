---
level: advanced
---

# Serde, units and systems of units

## serde

With the feature `serde` the numbers are `Serialize` / `Deserialize`, `no_std` and without alloc (STD-94): the same
rule as the wire, each value as the number it stores, and a field marked `with_unit` as text in its unit (`"1.5
ms"`). See the book's [serde: configs and other formats](https://exact.vhrd.tech/wire/shrink_wrap/#serde-configs-and-other-formats).

## Units

The unit markers are grouped by cargo feature, each with its own dimension, symbol and exact scale; `Per`, `Times`
and `Sqrt` build the rest. See the book's [Units and dimensions](https://exact.vhrd.tech/start/units/).

Beside SI there are two other systems of units behind `ww_si`'s feature `systems` (on with `std`): Gaussian (cm, g, s;
the gauss, oersted, statcoulomb, erg, dyne) and Natural (ħ = c = 1; the electronvolt for energy, mass and inverse
length). Their units go into an API like any other, `Fixed<Gauss, Milli, i16>` is a magnetometer in mG, and values of
two systems never add or convert by accident: a value crosses into SI only by `Fixed::to_system::<Si>()` and a
`convert` into a unit of SI, with a factor `exact_units` knows (μ₀ through a constant set where Gaussian needs it).
See the book's [Systems](https://exact.vhrd.tech/maths/systems/) and, for the model, [Systems of
units](units_in_the_model.md#systems-of-units).

The wire format is the feature `shrink_wrap`, on by default through `ww_si`. Without it `exact_units` has no
dependencies and its types are plain Rust (a `Dim`'s exponents are halves in an `i8`): code that only checks units at compile
time builds without the wire format, and the bytes are the same whichever crate turns it on.
