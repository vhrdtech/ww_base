# AGENTS.md

Guidance for AI code assistants working in this repository.

## What this is

ww_base holds the base types and identity crates of [WireWeaver](https://github.com/vhrdtech/wire_weaver):
quantities (`ww_si`), numbers with their type (`ww_numeric`), versions and `GlobalTypeId` (`ww_version`), the global
id ledger (`ww_global`), common errors (`ww_error`), dates (`ww_date_time`), keyed array wire types (`ww_keyed`) and
the CRCs (`ww_crc`). They stand on [shrink_wrap](https://github.com/romixlab/shrink_wrap) (the wire format) and
[exact_units](https://github.com/romixlab/exact_units) (the units), both by git, and on nothing of WireWeaver.
WireWeaver, its standard library and tools take these crates by git.

Read `docs/ww_si.md` and `docs/exact_units.md` first, then the page of the crate you touch (`ww_numeric.md`,
`ww_version.md`, `ww_error.md`, `ww_date_time.md`) and for units `units_in_the_model.md`, `points.md`, `indices.md`,
`rounding.md`, `magnitude.md`, `customary.md`. The docs are published in the standard library section of the
WireWeaver book, which assembles them from this repository: links to other pages of the book (`../api/..`,
`../serdes/..`) only resolve there. Prefer the docs to the source; where they are unclear, the source and its
comments are authoritative.

## Layout

| Path | What |
| --- | --- |
| `ww_si/` | `Quantity<U>`, `AnyQuantity`, `SIExpr`, `Timed`, `Unwrapper` / `TimeUnwrapper`, `RunScale`, `units::TABLE`, `tree` |
| `ww_numeric/` | `NumericValue`, `NumericBaseType`, `NumericAnyType`, `NumericValueStore`, `Integer` |
| `ww_version/` | `Version`, `FullVersion`, `CompactVersion`, `WireVersion`, `ImageFacts`, `GlobalTypeId`, `ApiHash` |
| `ww_global/` | `wire_weaver_gid.json` (the ledger), `assigned_ids.lock`, its build script, `trait_hash` |
| `ww_error/` | `CommonError` |
| `ww_date_time/` | `DateTime`, `NaiveDate`, `NaiveTime` |
| `ww_keyed/` | `KeyEvent` |
| `ww_crc/` | CRC statics, table size by feature |
| `docs/` | the pages of these crates |

Each crate but `ww_crc` keeps its API snapshots in `api_snapshots/` (`superseded/` holds replaced definitions).

## Layers

These crates are layer 2 of WireWeaver's crate layers: below the API description, the framework and the standard
library. No dependency on a WireWeaver crate, dev-dependencies included: no `ww_api`, no `#[ww_trait]`, no
`wire_weaver_evolution` test (`ww api check` in `just lint` stands in for it). A type that needs one of them belongs
upstairs. A crate here may depend on another crate here without a cycle (`ww_error` -> `ww_si` -> `ww_numeric`).

## FEATURES.md

`FEATURES.md` is the feature tracker and roadmap, the only place status is kept. Check it before starting a feature
and update the matching item in the same change when finishing one (conventions at its top).

- IDs are stable per area (`STD-3`, `EVO-16`, `TR-31`): never renumber or reuse one; new items take the next free
  number of their area, which is WireWeaver's id space. Use them in commits, CHANGELOG entries and
  `// TODO(STD-74): ...`.
- When talking to the user, name an ID with a 2-4 word kebab-case slug from its title: `STD-38 one-generic-quantity`.

## Commands

```sh
just test --changed   # the crates changed against main and their dependents; before a session's work is ready
just test             # everything: nextest, doctests, firmware feature sets, just check
just lint             # fmt, cargo sort -w -g, typos, clippy and rustdoc -D warnings, then just api-check
just api-check        # `ww api check` of every API crate (ww from PATH, or WW=path); skipped with a note without ww
just check            # ww_si feature sets, every crate without std on Cortex-M0
just save-snapshots   # `ww api save` of every API crate; `--force` only for a version never published
just gid-lock         # append ids newly assigned in wire_weaver_gid.json to assigned_ids.lock
```

Single test: `cargo nextest run -p ww_si <test>` (nextest is required). Tests run with default features (`defmt`
breaks host linking). Plain `cargo sort` flattens grouped tables: always pass `-w -g`. Project words go into
`typos.toml`. CI (`.github/workflows/ci.yml`) fails on any warning; it has no `ww`, so it skips `just api-check`.

## Numbers are typed

A number in a type that goes on the wire is typed, not a bare integer with its meaning in its name: a physical
quantity is an `exact_units` / `ww_si` type, a count a counted unit, an identifier a newtype, a position or sequence
number `Index` / `IndexFrom1` / `Seq`. `Fixed` and a one-field newtype are exactly their `T` on the wire. A bare
integer stays only for a raw protocol field or a truly dimensionless scale.

## Snapshots, wire versions and downstream

- The bytes are a contract, and so is each type's shape in the API model. A change of a type is checked by
  `ww api check <crate>` against the newest snapshot: a wire break bumps the crate's `wire = "wN.M"` line in
  `[package.metadata.wire_weaver]` and needs an `EVO-n` item; a breaking Rust or model change bumps the minor (pre-1.0).
- A snapshot of a published version never changes. One of a version never published is refreshed in place with
  `just save-snapshots --force`; say so in the commit.
- Claims about bytes in docs are never hand-derived: run a temporary example (`cargo run --example scratch_*`), copy
  the output, delete the example.
- WireWeaver checks these types against its conformance corpus (`tests/conformance` there) and its evolution
  fixtures. Before a wire- or model-relevant change lands, build WireWeaver against it with a
  `[patch."https://github.com/vhrdtech/ww_base"]` in that worktree's `.cargo/config.toml` (never committed) and run
  its `conformance`, `quantities` and `stdlib_evolution` tests.
- A new trait id goes into `ww_global/wire_weaver_gid.json` here first, then `just gid-lock`; ids are never reused
  (`ww_global/README.md`).

## Firmware: flash and cycles

The types run on small MCUs. A device enables only the quantity groups and number types it uses (cargo features of
`ww_si` and `ww_numeric`); `std` turns them all on.

- No float code where a firmware did not enable `f32`, no 64- or 128-bit code with number types up to 32 bits
  (`just test` runs those feature sets). WireWeaver's flash and no-float gates measure the effect on real images.
- No feature changes bytes: a cargo feature may trade size for speed, never the wire format.

## Proper fixes, never a lost capability

- A bug found on the way is fixed where it lives, as its own commit, with a test that fails without it.
- A removal, replacement or refactor keeps everything that worked before. Never narrow tests, docs or
  `FEATURES.md` to fit a loss.
- Flaky and slow tests: find the cause, never raise a limit (`.config/nextest.toml` holds hang detectors only).

## Changelogs, commits and versions

- Every user-visible change gets a line under `## Unreleased` in the crate's `CHANGELOG.md` and a summary line in the
  root `CHANGELOG.md`, in the same change, with the item id (`### ⚠️ Breaking`, `### 🚀 Features`, `### 🐛 Fixes`,
  one section per kind; for breaking changes say what users must change).
- Commits use Conventional Commits with a scope (`feat(ww_si): ...`): a short imperative subject, a blank line, a
  body of at most 5 lines saying what and why, and a `Tested:` line. A move is never edited in the same commit.
- Work commits never bump versions; the release commit does. Each crate has its own version, which for a crate with
  snapshots is its API identity: never skip one.
- Dependencies live in the root `[workspace.dependencies]`; crates inherit `authors`, `edition`, `rust-version`,
  `license`, `repository` and `[lints]`. A workspace entry with `default-features = false` makes members name the
  features they need.
- Public text (README, docs, CHANGELOG, commits, comments) is neutral: no private paths or internal tracker names.
