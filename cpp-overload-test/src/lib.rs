//! Test C++ overloads that can be successfully represented in Rust.
//! Each type in the C++ standard library has a module under `stdcpp`.

#![feature(splat, tuple_trait, c_size_t)]
#![allow(incomplete_features)]

/// The C++ standard library.
pub mod stdcpp {
    // Add new modules for each type here, in alphabetical order.
    pub mod forward_list;
}
