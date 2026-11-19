# ww_base changelog

The base types and identity crates of WireWeaver were developed in the wire_weaver repository (part of the time
under `ww_stdlib/`) and moved here in November 2026 with their history, API snapshots, docs pages and feature items.
Each crate keeps its own detailed changelog (`<crate>/CHANGELOG.md`); this file has one summary line per change.

## Unreleased

### ⚠️ Breaking

- Repository: ww_crc, ww_date_time, ww_error, ww_global, ww_keyed, ww_numeric, ww_si and ww_version live in vhrdtech/ww_base; take them by git from there.
- shrink_wrap and exact_units come by git from romixlab/shrink_wrap and romixlab/exact_units.

### 🐛 Fixes

- ww_error: the crate doc example builds again (STD-16).
- ww_global: a ledger id above `u32::MAX` fails the build instead of wrapping (API-20).

