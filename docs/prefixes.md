---
level: advanced
---

# Prefixes

`Prefix` covers every SI prefix from `Quecto` (10⁻³⁰) to `Quetta` (10³⁰), the binary ones `Kibi` to `Tebi` (2¹⁰ to
2⁴⁰), and three families of exact scales without a name, each with one small parameter: `Pow10` (a power of ten
without a prefix, 10⁻⁴), `Pow2` (a Q format's 2⁻¹⁵) and `FullScale` (a full-scale fraction, 1/65535 for 16 bits). So a
`Quantity` holds the values of the `Fixed` scales firmware uses most, exactly; a family never holds a named prefix's
scale (`Pow10(-3)` is refused, it is `Milli`), so equal scales are equal prefixes. Other exact scales (π/2¹⁵, 1/5) are
no `Prefix`: such a value is an [`AnyQuantity`](runtime_units.md).

| Value | `{}` | `{:#}` | Bytes |
|---|---|---|---|
| `Volt::milli(3300u16)` | `3300 mV` | `3.3 V` | `12 e4 0c` |
| `Quantity::<units::Byte>::new(Prefix::Kibi, 64u32)` | `64 KiB` | `65536 B` | `93 30 40 00 00 00` |
| `Second` of 12345u32 at `Pow10(-4)` | `12345·10⁻⁴ s` | `1.2345 s` | `d3 fc 30 39 30 00 00` |
| `Second` of 32768u32 at `Pow2(-15)` | `32768·2⁻¹⁵ s` | `1 s` | `e3 f1 30 00 80 00 00` |
| `Ratio` of 32768u32 at `FullScale(16)` | `32768·1/65535` | `0.5000076295109483` | `f3 10 30 00 80 00 00` |
| `Ratio` of 5u32 at `Unknown(1000)` | `5·<scale 1000>` | | `8d f1 30 05 00 00 00` |

The most used prefixes (`Unit`, `Milli`, `Micro`, `Nano`, `Pico`, `Kilo`, `Mega`, `Giga`) take one nibble on the wire,
the other named ones a byte, and the three families one more byte for their parameter, so a scale takes 2 bytes at
most and ordinary values pay nothing for the others. In RAM a `Prefix` is four bytes, on a host and on M0, M4F and
M33 alike, and a `Volt` twelve (as before the unnamed scales came in).

A newer version may add kinds of scale; each is tag-only, its numbers packed into the `UNib32` tag. An older reader
gets it as `Prefix::Unknown(tag)` (a `#[future_version]` catch-all, see
[the derive reference](../serdes/derive_reference.md#no_future_version-future_version-name-enums-only)), skips the value and reads the rest of the message; nothing
converts, adds or compares such a value except to one of the same scale, and `{}` shows it as `·<scale N>`.

`pow10()` is the power of ten (`Some(-3)` for `Milli`, `Some(-4)` for `Pow10(-4)`, `None` for `Kibi` or 1/65535),
`magnitude()` the exact scale of any prefix but `Unknown` (`None` there), `Prefix::from_magnitude(&m)` the prefix of an
exact scale if there is one, `symbol()` the symbol of a named prefix (`m`, `µ`, `k`, `Ki`). Arithmetic across two
decimal prefixes stays on powers of ten; only a binary prefix or a family goes through `Magnitude`.
