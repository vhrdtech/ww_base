---
level: advanced
---

# Non-linear codes

A `codec::Coded<C, V, T>` is a number given by a non-linear code instead of a linear step: a G.711 μ-law byte, a
gamma-encoded pixel. On the wire it is its code `T`; the codec `C` is a marker whose definition the model carries
(`MarkerDef::Codec`, exact_units' `CodecDef`: an id and a shape), so a client decodes and encodes it without the
`Codec` trait or a generated type (STD-109). Spell out all three arguments in an API, the walker doesn't follow the
trait's associated types:

```rust
use ww_si::{Fixed, codec::{Coded, MuLaw}, prefix::Pow2, units::Ratio};

#[ww_api_root]
trait Line {
    /// One G.711 μ-law byte: the 14-bit linear sample it means is the `Fixed`
    property!(rw voice: Coded<MuLaw, Fixed<Ratio, Pow2<-13>, i16>, u8>);
}
```

The dynamic client, `ww` and Python read such a value as the `Fixed` it means: the byte 0x80 is the sample 8031.
Writing a quantity encodes it by the codec, exactly as a write into a `Fixed` is exact: a value between two codes is
refused, naming the nearest one.

```
31·2⁻¹³ is no code of exact_units::MuLaw: the nearest is 30·2⁻¹³ (code 240)
```

Python's `ww.Quantity` has the code it was read as in `code` (128 for 0x80) beside the linear `value`. In Rust,
`wire_weaver_client::quantity::coded_of(&ty, &bundle)` reads the type as a `CodedType` (the codec's definition, the
linear `FixedType`, the code's storage), and `ww_self::units::codec_decode` / `codec_encode` run a definition. The
walker reads every shape from the codec's source: `MuLaw`, `ALaw`, `Table` (CAN FD's data length code),
`Offset` (video's narrow-range quantizers) and `MantissaExponent` (the streams' scales). A table the definition
names is the array literal of a `const` in the codec's module, and its numbers are literals or `+`, `-`, `*` and
`<<` of them: the walker evaluates no `const` code, so a table computed in one is refused, naming it. A codec whose
shape has no algorithm yet (`ALaw`) is shown as its code.

## Transfer curves

A video code has two layers: a quantizer takes the signal E′ to a code, and a transfer curve says what light E′
means. `codec::Transfer<Q, C>` puts them together, the quantizer `Q` (an `Offset` codec such as
`video::Narrow10`) with the curve `C` (an `impl Curve`: `video::Pq`, `video::Hlg`, `video::Bt709`) above it
(STD-112). On the wire it is still `Q`'s code; `Coded<Transfer<Narrow10, Pq>, ..>` is a 10-bit PQ pixel in two
bytes:

```rust
use ww_si::codec::{Coded, Transfer, video::{Narrow10, NarrowSignal10, Pq}};

#[ww_api_root]
trait Display {
    /// A pixel's luma: the signal E′ is `Narrow10`'s, the light it means is PQ's
    property!(rw luma: Coded<Transfer<Narrow10, Pq>, NarrowSignal10, u16>);
}
```

`Transfer` is generic, so its definition is one per instance. The model holds three markers: `Transfer` itself
(`MarkerDef::Transfer`, the variant alone), the quantizer's codec (`MarkerDef::Codec`) and the curve
(`MarkerDef::Curve`, exact_units' `CurveDef`: the light's unit and scale, the signal's domain and the exact
segments). `ww_self::units::codec_of` puts an instance together as `Transfer`'s `DEF` does, `Q`'s definition with
`curve` set to `C`'s, so the client reads the code as `Q`'s signal: code 940 is E′ = 876/876.

The light a code means comes from the curve. In Rust, `CodedType::light(raw)` (and `ww_self::units::codec_light`)
gives a `Light`: the unit by its marker (`Nit` for PQ), scene or display light, the exact value where it is a
rational, and the curve's float edge. PQ's code 940 is exactly 10 000 cd/m² and 64 exactly 0; every code between
has an irrational light, only the float edge (`exact` is `None`). A code outside the curve's domain (footroom below
black) has no light. Python's `ww.Quantity` has `light` (the float), `light_exact` (a `fractions.Fraction`, or
`None`) and `light_unit` (`"Nit"`). Writing goes through the signal: a quantity is written as its exact code, as
any `Coded` is.

The walker reads a curve from its source, `impl Curve for Pq { const DEF: CurveDef<'static> = Pq::DEF; }`: the
constants the definition names (`Pq::M1`, a module's `const PQ_FORM: CurveForm = ..`, `Self::` in an inherent
`impl`) are put in, and `Fraction`'s checked arithmetic (`Hlg::A.checked_div_int(3)`, through an unwrap helper
of the module such as `known(..)`) runs with exact_units' own `Fraction`, exactly as the compiled `DEF` computes
it. Any other call is refused, naming it. A generic `impl<..> Codec` of another crate, whose definition is put
together by code the walker doesn't run, is refused where an API uses it; `Transfer` is the one generic codec the
model knows.

The numbers of a curve above a codec (`CodecDef::curve`, its segments' gains, offsets and exponents) are
`exact_units::Fraction`s, `±num/den · 10^exp` with `exp` the value's power of 5: one canonical form per value, so
1/800 is `{ neg: false, num: 1, den: 8, exp: -2 }`. On the wire a `Fraction` is its four fields (`final_structure`),
37 bytes. A form that isn't the canonical one (`10 · 10⁰` for `1 · 10¹`) doesn't decode in Rust; the dynamic client
and Python read and write a `Fraction` by its fields without that check, which is Rust code the model can't carry
(SW-20), so a device refuses a non-canonical one they send. The conformance corpus's `stdlib.si.fraction.*` cases
hold the bytes.

A codec may name spare codes, codes that mean no value (`CodecDef::spare`, a list of `CodeSpan { low, high }`):
`MuLawSingleZero` is μ-law with 0x7F spare, where `MuLaw` reads it as a second zero. A spare code read from the
wire is refused, by the typed `Coded` and by every client from the definition in the model, with the code named:

```
127 is a spare code of MuLawSingleZero, no value (exact_units::Coded<MuLawSingleZero, Fixed<Ratio, Pow2<-13>, i16, Unbounded>, u8>)
```

`codec_decode` gives `None` for one, as `CodecDef::decode` does, and `codec_is_spare` asks. A spare code is where
an `exact_units::niche::Niched` puts its own codes: `Niched<Coded<MuLawSingleZero, ..>, Null<0x7F>, u8>` is an
`Option` of a μ-law sample in one byte, 0x7F the `None`. The model has no niche wrapper yet, so a client reads a
`Niched` as its raw code.
