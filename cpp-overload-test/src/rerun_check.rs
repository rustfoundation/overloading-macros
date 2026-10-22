//! Checks that new or changed `cpp!` blocks get compiled without needing `cargo clean`.
//!
//! `rerun-check.sh` runs the tests in this file, then swaps it for `rerun-check/rerun_check.rs`,
//! and runs the tests again. The second run only passes if `build.rs` reruns `cpp_build`
//! when a file under `src/` changes.

use cpp::cpp;

/// A value from an existing `cpp` block, which the replacement file changes.
fn changed_value() -> i32 {
    unsafe { cpp!([] -> i32 as "int" { return 1; }) }
}

#[test]
fn test_rerun_check() {
    assert_eq!(changed_value(), 1);
}
