//! Test C++ [std::multimap][multimap] overloads that can't be represented in Rust,
//! and record the macro diagnostics.
//! Error output is in [stdcpp-multimap.stderr].
//! Working overloads are in [multimap.rs].
//!
//! [multimap]: https://en.cppreference.com/w/cpp/container/multimap
//! [stdcpp-multimap.stderr]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/examples/stdcpp-multimap.stderr
//! [multimap.rs]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/src/stdcpp/multimap.rs
//!
//! The `cpp!` macro doesn't work here, because of technical limitations it can only run one build
//! per crate. (Examples and binaries are built separately from `lib.rs`.)
//! To add C++ code here, use /* ... */ code comment blocks, or just skip it.

#![feature(splat, tuple_trait, c_size_t)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;
use std::ffi::{c_int, c_size_t};

/// Workaround for missing generic support in the `overload!` macro.
type K = c_int;

/// Workaround for missing generic support in the `overload!` macro.
type V = c_int;

/// A fake multimap wrapper used to test overload resolution.
/// The C++ container is represented by a Rust vector for type checking.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdMultimapA(Vec<(K, V)>);

// Accessors: `find(...)`
overload! {
    impl StdMultimapA {
        /// Find an element with the key, from a shared reference.
        ///
        /// ### Incompatibilities
        ///
        /// The `overload!` macro can't overload on `&self` vs `&mut self`,
        /// because all overloads must have the same receiver kind.
        ///
        /// This also affects other const and non-const accessors, such as
        /// `lower_bound`, `upper_bound`, `equal_range`, `begin` and `end`.
        fn find(&self, _key: K) -> Option<&(K, V)> {
            todo!()
        }

        /// Find an element with the key, from a mutable reference.
        fn find(&mut self, _key: K) -> Option<&mut (K, V)> {
            todo!()
        }
    }
}

/// A wrapper struct to demonstrate key and iterator-position collisions.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdMultimapB(Vec<(K, V)>);
struct StdMultimapIter;

/// Workaround for missing generic support in the `overload!` macro.
/// Models a C++ iterator as an integer position.
type Pos = K;

// Modifiers: `erase(...)`
overload! {
    impl StdMultimapB {
        /// Erase every element whose key equals `key`.
        fn erase(&mut self, _key: K) -> c_size_t {
            todo!()
        }

        /// Erase the element at `pos`.
        ///
        /// ### Incompatibilities
        ///
        /// This overload has the same types as `erase(key)` when the iterator
        /// is modelled as an integer, but different semantics:
        /// - `erase(key)` removes every element with that key.
        /// - `erase(pos)` removes one element.
        fn erase(&mut self, _pos: Pos) -> StdMultimapIter {
            todo!()
        }
    }
}

/// A wrapper struct to demonstrate the workaround for different receiver kinds.
struct StdMultimapC(Vec<(K, V)>);

// Workaround: associated functions with an explicit `this` argument.
overload! {
    impl StdMultimapC {
        /// Find an element with the key, from a shared reference.
        ///
        /// ### Workaround
        ///
        /// Uses an associated function instead of a method, with `&'static`
        /// to avoid generic lifetimes. This is less ergonomic than a normal method.
        fn find(_this: &'static StdMultimapC, _key: K) -> Option<&'static (K, V)> {
            todo!()
        }

        /// Find an element with the key, from a mutable reference.
        fn find(_this: &'static mut StdMultimapC, _key: K) -> Option<&'static mut (K, V)> {
            todo!()
        }
    }
}

pub fn main() {}
