//! Test C++ overloads that can be successfully represented in Rust.
//! Each type in the C++ standard library has a module under `stdcpp`.

#![feature(splat, tuple_trait, c_size_t)]
#![allow(incomplete_features)]
// Some long C++ code blocks require more macro recursion depth.
#![recursion_limit = "512"]

/// The C++ standard library.
pub mod stdcpp {
    // Add new modules for each type here, in alphabetical order.
    pub mod forward_list;
    pub mod hash;
}

// Checks that `build.rs` rebuilds changed `cpp!` code, see `rerun-check.sh`.
#[cfg(test)]
mod rerun_check;

// Checks that all the C++ classes in our list are available in CI.
#[cfg(test)]
mod stdcpp_classes;
