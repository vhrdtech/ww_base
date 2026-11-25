---
level: advanced
---

# Units in ww, Python and dynamic clients

## Quantity in clients

A client that only has a device's introspection data still knows a quantity for what it is: `ww introspect` and
`ww observe` print `3300 mV`, Python gets a `ww.Quantity`. The unit comes from the marker: a marker of `ww_si::units`
by its name through `units::by_name` (symbol, dimension and scale), `Per<A, B>` and `Times<A, B>` as a tree of their
parts (`0.00012 N·m/(rad/s)`), a unit of your own API crate (`unit!`) by the definition the model carries (INT-13,
its identity, symbol, dimension and exact scale), and a hand-written marker without one by its name and docs only.
This works for ww_si 0.2 and later.

```python
q = dev.supply.read()        # Quantity(3300 mV U16)
str(q), float(q)             # "3300 mV", 3.3
q.symbol, q.dim              # "V", {"kg": 1, "m": 2, "s": -3, "A": -1}
q.to_si()                    # 3.3
dev.supply.write(1.5)        # 1.5 V
```

In Rust, `wire_weaver_client::quantity::DynQuantity::from_value(&value, &ty, &bundle)` gives the prefix, the number
and a `DynUnit`; its `Display` is the same text as the typed `Quantity`'s, `to_si_f64()` converts through the unit's
scale and `to_value()` makes a value to write. See `wire_weaver_py/README.md` for the Python side.

An [`AnyQuantity`](runtime_units.md) goes the same way: `DynQuantity::from_value` reads its unit from the
value, a reference through the bundle (`ww_self`'s `ApiBundleOwned` is a `UnitTable` of every unit the API uses) and
then `ww_si::units::TABLE` (`quantity::units_of`). A named unit of ww_si becomes the same `DynUnit` as the typed
quantity's (so `3300 mV` either way), a device's own unit keeps its definition, and a reference no table lists is a
`DynUnit::Ref`, shown by its name. `ww`, Python and `to_value()` treat it like any other quantity; `to_value()`
compacts the unit against the bundle, so the API's units go out by reference. A capture (`ww tap`) stores the
device's bundle, so its values decode with no API crate at hand.
`DynUnit` is shown and converted by `ww_si::tree`, the code behind the typed `Display`: a client with its own unit
model implements `ww_si::tree::Tree` for it and gets the same text.

## Fixed in clients

A client that only has a device's introspection data knows a `Fixed` for what it is: its unit, its scale, the number
type it is stored in and its bounds. `ww introspect` (and `ww api tree` from source) shows what the numbers are in
and the bounds after the type:

```
├─ 5 rw property timeout: Fixed<Second, Milli, u16, Unbounded> [ms]
├─ 7 rw property window: Fixed<Second, Milli, u16, Range<1, 5000>> [ms, 1..=5000]
├─ 15 rw property uptime: Fixed<Second, Micro, u64, Unbounded> [µs]
├─ 16 rw property buffer: Fixed<Byte, Kibi, u16, Unbounded> [KiB]
```

`ww observe`, `ww stream`, `ww tap` and `ww explain` print a value as the typed `Fixed` prints itself, exactly:
`1730000000000000 µs` from a `u64`, never through a float, `4 KiB` for a binary scale, the scale's own symbol for a
`scale!` of the API's crate. Values written as expressions (`ww explain 'uptime = q!(5 ms)'`, the dynamic client's
`expr::parse_value`) take a quantity in `q!(..)` and convert it exactly into the stored scale and number type:
`q!(5 ms)` into the `uptime` above stores 5000, `q!(0.1 ms)` 100, `q!(1 MiB)` into `buffer` 1024, `q!(8192 B)` 8.
What doesn't fit is refused before anything is sent, with the type named:

```
1.5 ns can't be written into Fixed<Second, Micro, u64, Unbounded>: not a whole number of µs
70 s can't be written into Fixed<Second, Milli, u16, Unbounded>: 70000 is out of range for u16
6 s can't be written into Fixed<Second, Milli, u16, Range<1, 5000>>: 6000 is out of 1..=5000
```

A plain number is the stored number (`uptime = 5000` is 5000 µs), what a script wrote into the field when it was a
plain `u64`, checked against the storage and the bounds the same way.

A float `Fixed` is never NaN unless its bound is `MaybeNan`, as in `exact_units` itself: every other bound,
`Unbounded` included, refuses NaN when a client writes it and when it comes off the wire. ±∞ is a value. A float
without a literal is written as its type's constant, `f32::NAN`, `f32::INFINITY` or `-f64::INFINITY`, which
`ww explain` also prints:

```
NaN is no value of Fixed<Volt, Milli, f32, Unbounded>: only a MaybeNan float holds NaN
```

A `Finite` float refuses ±∞ too, on the wire and when a client writes it (STD-104):

```
inf is no value of exact_units::Fixed<Volt, Milli, f32, Finite>: a Finite float is never ±∞
```
[Indices and sequence numbers](indices.md) are shown with what they count, `#1000 (Sample)`,
`#2 (Item, from 1)`, `seq 41 (Frame)`, and take their number, an `IndexFrom1` refusing 0.

Python gets a `ww.Quantity`, as for a [`Quantity`](#quantity-in-clients), with the stored
number and type:

```python
u = dev.uptime.read()        # 1730000000000000 µs
u.value, u.prefix, u.storage # 1730000000000000, "Micro", "u64"
u.symbol, u.unit, u.fixed    # "s", "Second", True
u.to_si()                    # 1730000000.0
dev.window.read().bounds     # (1, 5000)
dev.uptime.write(dev.timeout.read())  # 5 ms, stored as 5000
dev.uptime.write(5000)       # as stored: 5000 µs
dev.uptime.write(ww.Quantity("5 ms"))  # made from text, stored as 5000
```

A write takes a `ww.Quantity` of a unit of the same dimension, converted exactly, or the stored number; anything
else is a `ValueError` with the same text as above. `float("nan")` into a NaN-free float `Fixed` is refused the
same way, and bytes holding NaN raise `ww.WireWeaverError` on decode. `ww.Quantity("0.25 K")` or
`ww.Quantity("1.5", "Ratio")` makes one from text, so a script writes the same value to firmware that stores it in
another scale ([a host for several wire lines](../evolution/multi_line_host.md)).

In Rust, `wire_weaver_client::quantity::fixed_of(&ty, &bundle)` reads the type as a `FixedType` (unit, scale as a
`DynPrefix`, `Storage { base, bounds }`), `DynQuantity::from_value` reads a value of it, and `DynQuantity::to_value`
writes into it exactly or returns the error above. `quantity::DynCount` does the same for indices and sequence
numbers. The C consumer of the [conformance matrix](../serdes/conformance.md) reads a `Fixed` as its `T`.
