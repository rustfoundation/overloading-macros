//! The replacement for `src/rerun_check.rs` that `rerun-check.sh` swaps in.
//!
//! It changes the C++ code in the existing `cpp!` block and adds a new `cpp!` block, so the
//! second test run covers both kinds of change.

use cpp::cpp;

/// A value from an existing `cpp!` block, which this file changes.
fn changed_value() -> i32 {
    unsafe { cpp!([] -> i32 as "int" { return 2; }) }
}

/// A value from a new `cpp!` block.
fn added_value() -> i32 {
    unsafe { cpp!([] -> i32 as "int" { return 3; }) }
}

#[test]
fn test_rerun_check() {
    assert_eq!(changed_value(), 2);
    assert_eq!(added_value(), 3);
}
