# ww_base

[![CI](https://github.com/vhrdtech/ww_base/actions/workflows/ci.yml/badge.svg)](https://github.com/vhrdtech/ww_base/actions/workflows/ci.yml)

> Base types and identity of [WireWeaver](https://github.com/vhrdtech/wire_weaver): quantities, numbers, versions,
> dates, common errors, global ids and checksums, for `no_std` devices and hosts alike.

These crates sit between the wire format ([shrink_wrap](https://github.com/romixlab/shrink_wrap)) and the units
([exact_units](https://github.com/romixlab/exact_units)) below, and WireWeaver's API description, framework and
standard library above. They depend on nothing of WireWeaver, so a format, a tool or a firmware can use them without
it.

## Crates

| Crate | What |
| --- | --- |
| `ww_si` | quantities: `Quantity<U>` with the unit in the type, `AnyQuantity` with the unit at run time, `Timed`, the timestamp `Unwrapper`, unit tables |
| `ww_numeric` | `NumericValue`, `NumericBaseType`, `NumericAnyType`: a number with its type on the wire; `NumericValueStore`; `Integer` for exact comparisons without wide integers |
| `ww_version` | `Version`, `FullVersion`, `CompactVersion`, `VersionTriplet`, `WireVersion`, `ImageFacts`, `GlobalTypeId`, `ApiHash` |
| `ww_global` | the global id ledger (`wire_weaver_gid.json`) as consts and a table, and `trait_hash` |
| `ww_error` | `CommonError`: the errors every API can return |
| `ww_date_time` | `DateTime` with typed, bounded calendar fields |
| `ww_keyed` | wire types of keyed resource arrays (`keyed!`) |
| `ww_crc` | the CRCs of WireWeaver's framer, link, block, settings and stream crates, one table each, sized by feature |

Every crate but `ww_crc` keeps its API snapshots in `api_snapshots/`: WireWeaver's `ww api check` compares the source
with them, so a change that breaks the wire or the API needs a version bump.

## Documentation

The pages in [`docs/`](docs) describe the quantity, number, version, date and error types and how exact units appear
on the wire and in an API's model. They are published in the standard library section of the
[WireWeaver book](https://ww.vhrd.tech/), which assembles them from this repository, so their links to other parts of
the book work there. [`FEATURES.md`](FEATURES.md) tracks what is done and planned.

## Repository layout

One directory per crate (table above), `docs/` with the pages. `just --list` shows the recipes: `just test`,
`just lint` (with `ww api check` when WireWeaver's `ww` is installed), `just check`, `just save-snapshots`.

## Depending on these crates

They are taken by git until they are on crates.io; `Cargo.lock` pins the commit.

```toml
[dependencies]
ww_si = { git = "https://github.com/vhrdtech/ww_base", default-features = false, features = ["electrical", "u16"] }

[features]
default = ["std"]
std = ["ww_si/std"]
```

On a device, turn on only the quantity groups and number types it uses; `std` turns them all on.

## License

MIT, see [LICENSE](LICENSE).
