#!/usr/bin/env bash
# Checks that new or changed `cpp!` blocks get compiled after an edit, without running
# `cargo clean` or touching `build.rs`. See `src/rerun_check.rs` for details.
set -euo pipefail

cd "$(dirname "$0")"
ORIGINAL=src/rerun_check.rs
REPLACEMENT=rerun-check/rerun_check.rs
BACKUP=$(mktemp)
cp "$ORIGINAL" "$BACKUP"
trap 'cp "$BACKUP" "$ORIGINAL"; rm -f "$BACKUP"' EXIT

run_check() {
    local output
    if ! output=$(cargo test --all-features --package cpp-overload-test --lib rerun_check 2>&1); then
        echo "$output"
        return 1
    fi
    echo "$output"
    # A test filter that matches nothing still passes, so check that the test actually ran.
    if ! grep -q "test rerun_check::test_rerun_check ... ok" <<<"$output"; then
        echo "rerun_check::test_rerun_check didn't run" >&2
        return 1
    fi
}

echo "Testing the original cpp! code:"
run_check

# Make sure the replacement is newer than the last build, even with coarse file timestamps.
sleep 1
echo "Replacing the cpp! code, then testing again without cargo clean:"
cp "$REPLACEMENT" "$ORIGINAL"
run_check
echo "cpp! changes were rebuilt"