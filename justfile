# `just jobs=N <recipe>` caps build and test parallelism for that run; unset leaves CARGO_BUILD_JOBS and
# NEXTEST_TEST_THREADS as inherited, or at cargo's default (the CPU count)
jobs := ""
export CARGO_BUILD_JOBS := if jobs != "" { jobs } else { env_var_or_default('CARGO_BUILD_JOBS', num_cpus()) }
export NEXTEST_TEST_THREADS := if jobs != "" { jobs } else { env_var_or_default('NEXTEST_TEST_THREADS', 'num-cpus') }

# The WireWeaver command line that checks and saves API snapshots: `ww` from PATH, or the binary `WW` names
ww := env_var_or_default('WW', 'ww')

# Crates with API snapshots (`api_snapshots/`), checked by `ww api check` in `just lint`
api_crates := "ww_date_time ww_error ww_global ww_keyed ww_numeric ww_si ww_version"

# List all available targets
default:
    @just --list

# The tests. `just test`: all of them (test-all). `just test --changed`: only the crates changed against main and
# the crates that depend on them (through tpm when installed, otherwise the full suite)
test *flags:
    if [ "{{ flags }}" = "--changed" ]; then \
        if command -v changedtests >/dev/null 2>&1; then f=$(changedtests) && if [ -n "$f" ]; then cargo nextest run --workspace -E "$f"; fi; \
        else echo "changedtests not found; running the full test suite instead" >&2; {{ just_executable() }} test-all; fi; \
    else {{ just_executable() }} test-all {{ flags }}; fi

# Every test: nextest, the doctests, the firmware feature sets of ww_numeric, ww_si and ww_crc, and `just check`
test-all: check
    cargo nextest run --workspace --no-fail-fast
    # nextest does not run doctests
    cargo test --workspace --doc -q
    # a firmware's SeriesStore sizes, compare and conversions, with the number types up to 32 bits only
    cargo nextest run -p ww_numeric --no-default-features --features u32,i32,f32 --test store --test integer
    cargo nextest run -p ww_si --no-default-features --features time,electrical,u32,i32,f32 --test store --test narrow
    # ww_crc without std, at every table size (TR-31)
    cargo nextest run -p ww_crc --no-default-features
    cargo nextest run -p ww_crc --no-default-features --features table-256
    cargo nextest run -p ww_crc --features table-slice16

# cargo check what tests don't build: ww_si with no number type and with a few, every crate without std on Cortex-M0
check:
    cargo check --workspace
    # NumericValue variants are features
    cargo check -p ww_si --no-default-features
    cargo check -p ww_si --no-default-features --features=f32,u16,defmt
    # Fixed on a firmware: no number type of NumericValue beyond what it enables (u64 storage without the u64 feature)
    cargo check -p ww_si --no-default-features --features=time,information,counting,dimensionless,u16
    for c in ww_crc ww_date_time ww_error ww_global ww_keyed ww_numeric ww_si ww_version; do \
        cargo check -q -p $c --no-default-features --target thumbv6m-none-eabi || exit 1; done

# Format, manifests, spelling, clippy and rustdoc as CI runs them, warnings are errors; then `just api-check`
lint:
    cargo fmt --all --check
    cargo sort -w -g --check
    typos
    cargo clippy --workspace --all-targets -- -D warnings
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
    just api-check

# Each API crate's version and wire line against its newest snapshot (`ww api check`); skipped with a note without `ww`
api-check:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! command -v {{ ww }} >/dev/null 2>&1; then
        echo "skipped: ww api check, ww missing (install WireWeaver's ww, or set WW to its path)"
        exit 0
    fi
    for c in {{ api_crates }}; do {{ ww }} api check $c; done

# Append ids newly assigned in ww_global/wire_weaver_gid.json to its append-only lock (API-20)
gid-lock:
    GID_LOCK=update cargo nextest run -p ww_global assigned_ids

# Save the API snapshots of every API crate into its api_snapshots/; `--force` overwrites a never published version's
save-snapshots *args:
    for c in {{ api_crates }}; do {{ ww }} api save $c {{ args }} || exit 1; done

# A library has nothing to put on PATH: builds it with the deploy profile, as a dependent's release build would
install:
    cargo build --profile deploy --locked --workspace

# Bring this checkout up to main: refuses on local changes, pulls, builds
deploy:
    #!/usr/bin/env bash
    set -euo pipefail
    dirty=$(git status --porcelain)
    if [ -n "$dirty" ]; then echo "local changes, not deploying:"; echo "$dirty"; exit 1; fi
    git pull --ff-only
    just install
