---
level: basic
---

# Quantities with a prefix

A [`Quantity`] is a number with a unit in its type and a prefix in each value. The prefix says what 1 stands for: `Volt::milli(3300u16)` is 3300 millivolts, and `Volt::base(3.3f32)` is 3.3 volts. The API says only that a value is a voltage. Each device picks the prefix and the number type that suit its hardware, and every host reads them all as the same quantity.

Use a `Quantity` when the devices that serve one API differ in resolution. An ADC board may report millivolts in a `u16`, and a precision board microvolts in an `i32`. The thermometer does not need this, because its API fixes everything, so this page uses a voltage for its examples. [Numbers with units](ww_si.md) compares the two types.

## Making and showing a quantity

A constructor named after a prefix makes a quantity: `milli`, `micro`, `nano`, `pico`, `centi`, `kilo`, `mega` and `giga`. `base` makes one with no prefix, and `new` takes a prefix as an argument. The number can be any number type the firmware enabled, such as `u16`, `i32` or `f32`.

`Display` writes the number as stored, with its prefix. The alternate form `{:#}` writes the value in the unit itself, and `Debug` adds the unit's name and the number type. [`book_units.rs:quantity`] shows them with the conversions of the next section:

```rust
--8<-- "ww_si/tests/book_units.rs:quantity"
```

```text
--8<-- "docs/generated/ww_si/quantity.txt"
```

## Comparing and converting

A quantity compares by what it means, not by how it is stored. 1500 mV in a `u16` equals 1.5 V in an `f32`, and the first assertion in the section above checks it. Two integers compare exactly, even when their prefixes are far apart.

Conversions say what may go wrong. `to_f64` gives the value in the unit itself. `rescaled_exact` changes the prefix only when the result is exact: 3000 mV is exactly 3 V, and 3300 mV is not a whole number of volts, so it gives `None`. `to_in` returns the value in a chosen prefix and number type, and its arguments name what happens to a fraction and to a value that does not fit. [Converting and comparing quantities](conversions.md) lists them all.

## The bytes on the wire

A quantity is its prefix, then its number. The prefix and the number type share the first byte, one nibble each, and the unit takes no bits at all. [`book_units.rs:quantity_bytes`] checks the three bytes of 3300 mV:

```rust
--8<-- "ww_si/tests/book_units.rs:quantity_bytes"
```

--8<-- "docs/assets/quantity_bytes.svg"

*The first byte of 3300 mV holds the prefix and the number type; the next two bytes are the `u16` 3300.*

A `Fixed` of the same value is one byte shorter, which is the price of a device choosing its own scale.

## Where to go next

- [Bounds](bounds.md) limits a quantity to a range, as it limits a `Fixed`.
- [Converting and comparing quantities](conversions.md) covers rounding, overflow and the integer arithmetic of a small CPU.
- [Constructors and bytes of a Quantity](quantity_reference.md) lists every constructor and more byte layouts.
- [Prefixes](prefixes.md) lists every prefix, including the binary and the unnamed ones.
- [Unit algebra](unit_algebra.md) multiplies quantities, and the compiler checks the units of the result.
