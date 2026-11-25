---
level: basic
---

# Fixed numbers

A `Fixed` is a number whose type says what it measures. `Fixed<U, P, T, B>` has four parts, and the thermometer uses all of them:

| Part | Says | In the thermometer's `Rate` | In its `Gain` |
|---|---|---|---|
| `U` | the unit | `Hertz` | `Ratio`, a number without a unit |
| `P` | the step: what 1 stands for | `One`, steps of 1 Hz | `One` |
| `T` | the storage on the wire | `u16` | `u8` |
| `B` | the [[bounds]] | `Range<1, 1000>` | `Range<1, 16>` |

On the wire and in memory a `Fixed` is exactly its `T`. A timeout kept as `shared_us: u64` becomes `Fixed<Second, Micro, u64>` with the same bytes, and the [[introspection]] data then says microseconds instead of leaving you to trust a field name. The `exact_units` crate defines the type; [Numbers with units](ww_si.md) explains how it relates to `ww_si`.

## Declaring a Fixed

An API crate names each `Fixed` it uses once, as a type alias, and uses the alias in its traits. These are the thermometer's three, from [`book_units.rs:fixed_types`]:

```rust
--8<-- "ww_si/tests/book_units.rs:fixed_types"
```

`Temperature` is a `Point` of a `Fixed`. A point is a reading on a scale that starts somewhere, and [Temperatures and other points](points.md) explains why the thermometer needs one.

A value comes from `try_new`, which checks it, or from `bounded`, which panics on a value outside the bounds and fails the build in `const` code. `Display` shows the number with its unit, and `Debug` shows the type's meaning too. The lines below come from [`book_units.rs:fixed_values`]:

```rust
--8<-- "ww_si/tests/book_units.rs:fixed_values"
```

They print:

```text
--8<-- "docs/generated/ww_si/fixed_values.txt"
```

A `Gain` has no unit, so it prints as the plain number 8.

## The same bytes as the plain number

A `Fixed` costs nothing on the wire. [`book_units.rs:fixed_bytes`] writes four values and checks their bytes:

```rust
--8<-- "ww_si/tests/book_units.rs:fixed_bytes"
```

The 1500 µs timeout has the four bytes of the `u32` 1500, and the `Gain` has the one byte of the `u8` 8. Nothing about the unit, the step or the bounds is sent. Both sides of a link know them from the API, and the type checks them on each side.

--8<-- "docs/assets/same_bytes.svg"

*A temperature and the plain `i16` with the same value have the same two bytes. The unit, the step and the origin are in the type only.*

A `Fixed` therefore replaces a bare integer in a [[property]] at no cost. It also keeps the device and the host from disagreeing about what the integer means: both read the same type.

## Aliases for the common cases

The common units have aliases, so `Millis<u16>` is the same type as `Fixed<Second, Milli, u16>`. [`book_units.rs:aliases`] shows two of them:

```rust
--8<-- "ww_si/tests/book_units.rs:aliases"
```

The other aliases are `Secs`, `Nanos`, `Minutes`, `Hours`, `Days` and `Hz`, the byte sizes `Bytes`, `Kibibytes`, `Mebibytes` and `Gibibytes`, `Count<Of, T>`, `Percent`, `Permille`, `Ppb` and `Duty16`, the Q formats and the binary angles. A moment in POSIX time is a `Point` of `origins::UnixEpoch`, aliased `UnixSecs`, `UnixMillis`, `UnixMicros` and `UnixNanos`. The `exact_units` book lists them in its [aliases table](https://exact.vhrd.tech/start/first_steps/#aliases-for-the-common-ones).

## Where to go next

- [Bounds](bounds.md) shows what happens to a value outside the range, on the device and in your program.
- [Quantities with a prefix](quantity.md) covers the other type of `ww_si`, for devices that choose their own scale.
- [Units in properties, methods and series](units_in_an_api.md) declares `Fixed` values in an API trait.
- [Unit algebra](unit_algebra.md) multiplies and divides them, with the result's unit as a type.
