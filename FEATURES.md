# ww_base features and roadmap

This file is the single source of truth for what the ww_base crates support, what is being worked on and what is
planned, for humans and AI agents alike.

## How to use this file

- **Status** of each item:
  - ✅ done
  - 🚧 in progress or partially done (the note says what is missing)
  - 📋 planned
  - 💡 idea, not committed to
  - ⛔ blocked (the note says on what)
  - 🔍 probably done or obsolete, needs a check before closing
  - 🧪 done and tested on the host, the check on hardware is still open (the note says what to check)
  - 🐛 known bug, not yet fixed (the note says how to hit it)
- **Target** is the release an item is planned for (`v0.5` is WireWeaver's next release, which these items were
  planned with). No tag means not scheduled.
- **IDs** (`STD-3`, `EVO-16`, `TR-31`) are stable: never renumber or reuse one. New items take the next free number
  of their area. Use the ID in commit messages, CHANGELOG entries and code `TODO`s (`// TODO(STD-74): ...`). When
  talking to people, name an ID with a short slug from its title (`STD-3 ww-si-and-numeric`), never bare.
- The items came from WireWeaver's `FEATURES.md` with their IDs, where these crates were developed. The areas are
  WireWeaver's: an item here is one whose code lives in this repository's crates. IDs not listed here (`SW`, `DER`,
  `API`, `SRV`, `CLT`, `INT`, `TEST`, ...) and paths outside this repository's crates (`tests/quantities`,
  `tests/conformance`, `wire_weaver_client`, `docs/api/..`) are [wire_weaver](https://github.com/vhrdtech/wire_weaver)'s,
  or [shrink_wrap](https://github.com/romixlab/shrink_wrap)'s and [exact_units](https://github.com/romixlab/exact_units)'s
  where they name those crates. The walker, model and client sides of these types stay with WireWeaver.
- Items are grouped by area. Each area lists what works first (✅), then open items ordered by target.
- When finishing work, update the item in the same change: mark it ✅, move it up to the done items of its area and
  add a pointer (docs page, crate, test or example). Don't delete done items. Move items that turn out to be obsolete
  to a "Dropped and superseded" section with a one-line reason.
- New ideas go into the matching area as 💡. Small code-level gaps stay as `TODO` comments in the source; only those
  that limit users or block a feature get an item here.

## Base types and quantities (`ww_si`, `ww_numeric`, `ww_version`, `ww_date_time`, `ww_error`, `STD`)

- ✅ **STD-2 `ww_version`**.
- ✅ **STD-3 `ww_si`** and **`ww_numeric`**.
- ✅ **STD-4 `ww_date_time`**.
- ✅ **STD-16 Common errors** · `v0.5` — `ww_error::CommonError` (`Unsupported`, `InvalidArgument`, `InUse`, `Busy`,
  `WrongState`, `NoPower`, `Refused`, `Timeout`, `Fault`, bounded strings), re-exported as
  `ww_api::prelude::CommonError`, is an error every item of every API can return, beside the API's own error enum
  and without a variant in it: the protocol carries it as `ww_client_server::ErrorKind::Common` (wire `w2.0`), a
  handler returns `Result<T, wire_weaver::Error<'_, E>>` (`Common` or `Api`, `?` takes both through `api_error!`) or
  `Result<T, CommonError>` for an item without an error type, the client gets `CallError::Common` or
  `Error::common()`, `#[errors(..)]` lists them as `CommonError(Busy, InUse)` on any item. Tools read the event, no
  crate name is matched: Python `RemoteError.common`, `ww` prints its text, `CommonError::DOCS` gives names and doc
  lines. Every `ww_stdlib` API crate uses it; `ww_counters`, `ww_log`, `ww_timebase` and `ww_uart` lost their error
  enums, which held nothing else. The server side is SRV-63.
  `docs/ww_error.md`, `docs/api/errors.md#common-errors`, `tests/common_error`, `wire_weaver_py`
  `test_errors.py`.
- ✅ **STD-23 `ww_si` / `ww_numeric` extended** · `v0.5` — quantity groups and host helpers as cargo features,
  `Second` as a device timestamp (wrap rule, host `Unwrapper`), constructors, conversions and comparisons across
  prefixes and storage, `UVlq32` / `UVlq64` storage, `Baud` for baud rates (STD-13), `Ratio`, `Torque` and
  `Timed<T>` (EX-11). `docs/ww_si.md`,
  `docs/ww_numeric.md`. Zero-overhead typed quantities (`Volt<u16, Milli>`) are not part of it.
- ✅ **STD-38 `ww_si` 0.2: one generic `Quantity<U: Unit>`** · `v0.5` — every quantity is
  `Quantity<U> { prefix, value, PhantomData<U> }` with a zero-size marker per unit in `ww_si::units` and the 0.1 names
  as aliases (`KiloGram` → `Gram`); `Dim`, `Scale`, `UnitExpr`, `Unit` (own markers in API crates), `Per` / `Times`;
  constructors, `quantize`, conversions (`to_si_f64` through each unit's scale), `with_prefix`, `same_repr`,
  comparisons, `Display` / `Debug` / `defmt` written once; unit algebra checked by the compiler against a product
  table verified at compile time; `units::by_name`. Same bytes on the wire as 0.1. `docs/ww_si.md`,
  `ww_si/tests`; the 0.2.0 snapshot holds `Quantity`, every marker and `Per` / `Times`, so devices
  leave them out of their introspection data. Other crates' call sites moved to the new constructors; the motor demo's
  own helpers (`units.rs`) and `Load` units are the follow-up piece; `BaseUnit` is replaced by STD-37 (run-time units).
- ✅ **STD-37 `ww_si` units known only at run time** · `v0.5` — `SIExpr<'i>` { `Named { name, symbol, dim, scale }`,
  `Mul`, `Div`, `Pow` } (recursive through `RefBox`, API-44) and `AnyQuantity<'i> { prefix, value, unit }`: the
  marker model on the wire, `Dim` as seven 4-bit exponents (`sized`), `Scale` too. A marker's `UnitExpr` maps one to
  one onto an `SIExpr` (`Unit::SI_EXPR`, `SIExpr::of`, `SIExprOwned::from`; back `==` and `known()`), shown, named
  and converted to SI by the same code as typed units, walks bounded by `MAX_UNIT_DEPTH`. `ww_adc::ChannelInfo::unit`
  is one; `BaseUnit`, `SIExp`, `INib8P3` are removed. `docs/runtime_units.md` (byte
  layouts), `ww_si/tests/dynamic.rs`, `tests/io_adc`. Dynamic clients, `ww` and Python show an
  `AnyQuantity` like the typed quantity of its unit (CLT-31's `DynQuantity`, both rendered by the public `ww_si::tree`
  walk), a device's own unit with its dimension and scale; writes take any quantity whose unit has a dimension.
  `tests/quantities` (an `AnyQuantity` property through the in-process transport), `wire_weaver_py/tests`.
- ✅ **STD-43 `ww_si` without wide integers on small targets** · `v0.5` — comparisons, `with_prefix`, the exact
  arithmetic and the integer conversions of quantities work on `ww_numeric::Integer` (sign and a magnitude as wide as
  the widest enabled number type): with types up to 32 bits only `u32` code, no `i64` / `i128` and no `f64` without
  float types. `to_i32_in` / `to_u32_in` / `to_u32_wrapping_in`; `to_i128_in` only with 64- or 128-bit types. 6 KiB
  less flash on Portwright (thumbv6m, `opt-level = "z"`). `docs/conversions.md`,
  `ww_si/tests/narrow.rs` and `ww_numeric/tests/integer.rs` (run with 32-bit types only too).
  STD-102 replaced these methods with `to_in` / `rescaled` / `rescaled_exact` on exact_units' kernel.
- ✅ **STD-102 `ww_si` on exact_units' integer kernel** · `v0.5` — conversions, comparisons and bounds of `Quantity`
  run on exact_units' NUM-13 kernel (`PrefixRatio`, `Prefix::rescale` / `cmp_scaled`) at the width of the enabled
  number types, for every scale (SI, binary, `Pow10`, `Pow2`, `FullScale`); the `i128` / `f64` slow paths
  (`ops.rs`) are gone. One conversion names its policy: `to_in::<T>(prefix, Rounding, Overflow)` replaces
  `to_u32_in`, `to_i32_in`, `to_u32_wrapping_in` and `to_i128_in`; `rescaled(prefix, Rounding, Overflow)` and
  `rescaled_exact(prefix)` replace `with_prefix`. No float code on a device without the `f32` feature (a float
  target is a compile error there), `f64` on hosts only; no wire change (corpus unchanged, 8 new bounded-quantity
  cases). Results are those of the old arithmetic (`ww_si/tests/oracle.rs`, proptests against it). QEMU
  M3 (`qemu-si-convert`, `fw-z` / `fw-3` instructions, before → after, exact_units at 1819d9f): KiB→B 6145/6508 →
  596/299, FullScale→mV 7346/7553 → 921/582, `to_fixed` 4340/4548 → 100/81, same prefix 99/44 → 70/34, mV→V 154/71
  → 139/96, ms→µs 153/90 → 140/98, µs < ms 184/112 → 176/119, bounded decode 413/325 → 464/316, `steps` 342/71 →
  293/100. Flash of the bench 34.1 → 18.1 KB (`fw-z`), 47.8 → 27.6 KB (`fw-3`). Portwright `.text` 305,256 → 290,664
  B, rePort 402,548 → 392,812 B (measured at exact_units 15cd9f5). TEST-33 is a gate now. `docs/conversions.md`.
- ✅ **STD-107 `AnyQuantity` by reference** · `v0.5` — the run-time side of exact_units UNIT-7 (piece 4): an `AnyQuantity` carries a compacted `SIExpr`, a `Ref` by `MarkerId` for each unit a `UnitTable` lists (the API's model) and a `Def` otherwise; `From<Quantity<U>>` gives `Def`s, so a value reads alone anywhere. `to_si_f64`, `display` and `AnyQuantityOwned::to_fixed` take the table, `to_quantity::<U>()` resolves through `U`'s own definition, `compact_in` (no_std, an arena) and `AnyQuantityOwned::compact` / `resolve`; `Display` reads through `units::TABLE`. `Volt` by reference is 17 bytes of unit instead of 31. ww_si re-exports `UnitTable`, `UnitDef`, `MarkerId`, `CompactError`. A tape (`ww_tape`) with an `AnyQuantity` of an API's own `Rpm` decodes from the bundle stored in it (`tests/quantities` `tape_decodes_a_reference_from_its_bundle`). [Docs](docs/runtime_units.md).
- ✅ **STD-110 `Timed.t` as a device point** · `v0.5` — the wire_weaver side of exact_units PT-4 point 4: `ww_si::Timed<V, Tm = DeviceMicros<u32>>`, `t` a `Point<DeviceBoot, Fixed<Second, Micro, u32>>` (µs since boot in a `u32`, wraps every 71.6 minutes) instead of a `Second`, its storage's bytes alone on the wire (wire change EVO-51). `Tm` is any `ww_si::TimePoint`: a `Point` in seconds of a linear origin in integer storage (`DeviceMillis<u16>`, a 32768 Hz RTC as `Pow2<-15>`, `UnixMicros<u64>`), with `Full` (the point in 64 bits), `low`, `wrapping_add` and `steps` for the device; `TimeStep<Tm>` is its step. `TimeUnwrapper<Tm>` gets the full point back on the host, sharing its core with the dynamic `Unwrapper` (which stays for `Second` times: stream heads, observe times, `Clock::now()`). A batched stream picks its `Tm` with `batched(time = ..)` (API-61), typed clients hand items out with the full point (CLT-70), the dynamic client and Python as a `Second` / `Quantity` in the point's step. Store 11 bytes per `Timed<Ampere>` (12), 32 `Timed<Volt>` in 21 bytes (22). Conformance cases `stdlib.si.timed.default`, `.millis_u16`, `.series`; tests in `ww_si` (`time_unwrap`, `store`, `series`), `tests/streams_batched` (`slow_stream_in_millis`). [Docs](docs/timed.md).
- ✅ **STD-116 Run-time scales in APIs** · `v0.5` — the wire_weaver side of exact_units NUM-42: `ww_si` re-exports `RunScale` (an exact positive scale known at run time, `num / den · 2^pow2`, one canonical form, refused on decode otherwise), `Scaled<U, T>` (a number at such a scale, the unit in the type) and `Ticks<T>` (`Scaled<Second, T>`, a stream's own time base). The walker reads both as the derived structs, a `Scaled`'s unit its first type argument; the dynamic client and Python read and write them as those structs (showing a `Scaled` as a quantity is CLT-71). STD-77 stays for a scale that lives in another field or resource. Conformance cases `stdlib.si.run_scale.ntsc`, `.rejected`, `stdlib.si.ticks`; test `run_time_scales` in `tests/quantities`. [Docs](docs/run_time_scales.md).
- ✅ **STD-60 Angle, information and counted things as dimensions** · `v0.5` — `Dim<'i>` has rad and bit as
  base dimensions and up to four counted bases known by crate and marker name (`Item`, `Sample`, `Frame`, `Packet`,
  `Cycle`, `Block`, `Sector`, `Chunk`, `Char`, `Code`, feature `counting`, an API's own with `ww_si::counted!`;
  `Bit`, `Nibble`, `Byte`, `Halfword`, `Word`, `Doubleword`, feature `information`) and a root flag for half
  exponents (V/√Hz, `Dim::sqrt`): bytes don't convert into % or rad, `AngularVelocity` isn't `Hertz`,
  `Per<Sample, Second> × Second` is a `Sample`, bytes of a sample only through `Per<Byte, Sample>`; checked at
  compile time (`Unit::DIM`) and at run time (`SIExpr`, `wire_weaver_client::DynUnit`, Python `Quantity.dim`).
  `Torque` is J/rad (written N·m). Wire change EVO-16. `Dim` itself lives in `exact_units` (NUM-1).
  [ww_si dimensions](docs/dimensions.md), `ww_si/tests/quantities.rs`.
- ✅ **STD-59 Runtime `ww_si` improvements** · `v0.5` — `Quantity` on `exact_units`' units, one unit system.
  `Prefix` without a table (Roman, 28 Oct 2026): named SI and binary prefixes (`Kibi` .. `Tebi`), the families
  `Pow10(i8)`, `Pow2(i8)` and `FullScale(bits)` (`32768·1/65535`, a Q format, `Pow10<-4>`), and `Unknown(u16)` for
  scale kinds of a newer version (DER-25): 4 B in RAM on host, M0, M4F and M33, `Volt` 12 B as before; named prefixes
  keep their one nibble on the wire, a family takes one byte more. Other scales (a BAM's π/2¹⁵) go to `AnyQuantity`,
  which holds any `Fixed` exactly. `Quantity<U, B = Unbounded>` with bounds checked on construction and decode, `B`
  explicit in the model; `Quantity::try_from_fixed` and Fixed/Quantity parity proptests; conformance cases (derive,
  dynamic, Python; C gap API-26); docs ([`ww_si`](docs/ww_si.md#fixed-or-quantity)). The
  client-side bounds check is CLT-54.
- 📋 **STD-53 `Volt` with a `u16` value fails to decode on a device** — `I2cConfig { io_level:
  Some(Volt::milli(3300u16)), .. }` from the host gets `ArgsDesFailed` on the rePort B200A firmware, the same level as
  a `u32` value works (report-i2c-master bench run, 27 Oct 2026). Find where the `ww_si` value storage / numeric
  type differs between the host's writer and the no_std reader, fix it there, and add the case to the conformance
  corpus.
- 📋 **STD-74 The runtime side catches up** · `v0.5` — `Quantity`, `AnyQuantity` and `SIExpr` get the exact magnitude,
  the °C point / interval split, levels and sentinels, so `ww_si` never lags behind `exact_units`; the `Fixed` ↔
  `Quantity` parity test (STD-59) covers kinds, points, levels and sentinels too: same result or same refusal.

## Evolution of these crates' types and the id ledger (`ww_global`, `EVO`)

- ✅ **EVO-4 Global traits referenced by hash**, global type registry (`ww_global`): one id per `crate::Trait` for every
  public `ww_stdlib` trait, by hand in `wire_weaver_gid.json`, renames keep the id through `aliases`, ids never
  reused (`retired`), `assigned_ids.lock` append-only and checked; `ww api check` flags a changed id (API-20).
- ✅ **EVO-16 `ww_si::Dim` with angle, information and counted bases** · `v0.5` — the wire change of `Dim` that
  STD-60 brings, in `SIExpr` / `AnyQuantity` and in the `ww_si` snapshot: no longer `sized` but `final_structure`,
  nine 4-bit exponents (rad and bit appended to the seven SI ones), a root bit and four optional counted bases
  (crate and marker name); a non-canonical `Dim` is refused on decode. Volt is `d2 1f 00 00 00 00`. `Scale` gains
  `Exact` (an `ExactScale`, a degree is exactly π/180) in the same break. Conformance cases `stdlib.si.dim.*`
  (`tests/conformance/corpus.json`), [ww_si](docs/runtime_units.md).
- ✅ **EVO-36 `SIExpr` references** · `v0.5` — the wire change of STD-107: `SIExpr`'s leaves are `Def(UnitDef)` (variant 0) and `Ref(MarkerId)` (variant 1, new), so `Mul`, `Div`, `Pow` and `Sqrt` move up one place (2..5); an `AnyQuantity` a client writes names the API's units by reference. Conformance cases `stdlib.si.si_expr.sqrt`, `decibel_loss_per_hundred_metres` and `rejected` carry the mark; `si_expr.ref`, `ref_per` and `any_quantity.*` are new. No device or host uses the format.
- ✅ **EVO-51 `Timed.t` as a device point on the wire** · `v0.5` — the wire change of STD-110: `Timed<V>` was `t: Second` (a prefix nibble, a tagged `NumericValue`) and is `t: Tm`, the bytes of the point's storage; `ww_si` 0.5.0 on a new wire line, `w5.0`, the stdlib snapshots naming a `ww_si` type saved again. New conformance cases `stdlib.si.timed.default`, `.millis_u16` and `.series` (no sealed case changed).
- ✅ **EVO-58 Typed, bounded calendar fields of `ww_date_time`** · `v0.5` — the wire change of `ww_date_time` (wire line `w1.0` -> `w2.0`): `NaiveDate::month` and `day` are `IndexFrom1<Month, U4, Range<1, 12>>` and `IndexFrom1<Day, U5, Range<1, 31>>`, `NaiveTime::secs` is `Fixed<Second, One, U17, Range<0, 86_399>>` and `frac` `Option<Fixed<Second, Nano, U31, Range<0, 1_999_999_999>>>`, `OtherTimezone::FixedOffset::secs` is `Fixed<Second, One, I18, Range<-86_399, 86_399>>`. The bytes are the same, the new bounds are checked on decode (`SubtypeOutOfRange`), so a bound added is a break. Types embedding `DateTime` (`ww_board_info`, `ww_firmware_info`, `ww_timebase`) move to a new wire line with it. No device or host uses the format.

## Checksums (`ww_crc`, `TR`)

- ✅ **TR-31 `ww_crc`, one CRC crate** · `v0.5` — the CRCs of `ww_framer`, `ww_link`, `ww_block`, `ww_settings`
  and `ww_stream` are statics of `ww_crc` (`CRC32_ISO_HDLC`, `CRC32C`, `CRC16_USB`, `CRC16_IBM_SDLC`, `CRC8_SMBUS`),
  so a firmware links one table per algorithm, its size picked once with `table-256` or `table-slice16` (bitwise
  without; the crates' `std` features turn on `table-slice16`). `ww_link` no longer depends on `ww_block` for its
  UDP CRC-32. Docs: `docs/transport/ww_framer.md#checksum-and-tail`; tests in `ww_crc` at every table size.
