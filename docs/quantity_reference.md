---
level: advanced
---

# Constructors and bytes of a Quantity

## Constructors

| Constructor                                   | Value                                                        |
|-----------------------------------------------|--------------------------------------------------------------|
| `new(prefix, value)`                          | any enabled number type: `u16`, `f32`, `UVlq32`, ..          |
| `base`, `milli`, `micro`, `nano`, `pico`      | `Volt::base(3u8)` is 3 V, `Second::micro(100u32)` 100 µs      |
| `centi`, `kilo`, `mega`, `giga`               | `Hertz::kilo(400u32)` is 400 kHz, `Gram::kilo(5u8)` 5 kg      |
| `from_f32(x)`                                 | `f32` in the unit itself, `const`                            |
| `from_parts(prefix, value)`                   | `const`, for statics and tables                              |
| `quantize(x, prefix, storage)`                | an `f32` put into the representation an API documents        |
| `Ratio::percent`, `per_mille`, `ppm`          | `Centi`, `Milli`, `Micro`                                    |

Mass is in grams, so a prefix works on it like on any other unit: 5 kg is `Gram::kilo(5u8)`.

`quantize` is what a device does with a value it computed as a float: `Ampere::quantize(0.1523, Prefix::Milli,
NumericBaseType::I32)` is 152 mA, rounded to the nearest integer and saturated at the storage's range (a float storage
takes the value as is). It computes in `f32`, no `f64` code on a device.

## On the wire and in introspection

A quantity is a `final_structure` struct of its prefix and its value; the unit takes no bits. The bytes are the same
as in ww_si 0.1, where each quantity was a struct of its own, so recorded data and `Series` blobs still decode:

| Value                        | Bytes                |
|------------------------------|----------------------|
| `Volt::milli(3300u16)`       | `12 e4 0c`           |
| `Ampere::micro(-152000i32)`  | `26 40 ae fd ff`     |
| `Volt::from_f32(3.3)`        | `07 33 33 53 40`     |

In the API model a use of `Volt` is an instance of the generic `Quantity` with the marker `units::Volt` as its
argument (an alias is not a definition of its own). Both are in the `ww_si` snapshot, so a device sends a reference
to them, not their definitions.
