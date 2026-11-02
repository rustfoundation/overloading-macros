//! Test C++ [std::unordered_map][unordered_map] overloads that can't be represented in Rust, and
//! record the macro diagnostics. Error output is in `stdcpp-unordered-map.stderr`.
//! Working overloads are in `src/stdcpp/unordered_map.rs`.
//!
//! [unordered_map]: https://en.cppreference.com/cpp/container/unordered_map
#![feature(splat, tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

/// Workaround for missing generic support in the `overload!` macro: the map key type.
type K = std::ffi::c_int;

/// Workaround for missing generic support in the `overload!` macro: the mapped value type.
type V = std::ffi::c_int;

/// A fake key-value pair, matching the working `KeyValue` type.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
#[derive(Copy, Clone)]
struct KeyValue {
    key: K,
    value: V,
}

/// A wrapper struct to hold the returned C++ pointer.
/// We fake the C++ map pointer with a Rust vector to help with type checking.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdUnorderedMap(Vec<KeyValue>);

// Modifiers: `insert(...)`
overload! {
    impl StdUnorderedMap {
        /// Insert a copy of `value`, the C++ `insert(const value_type&)` overload.
        fn insert(&mut self, _value: KeyValue) -> bool {
            todo!()
        }
    }
}

// Accessors: `max_load_factor(...)`
overload! {
    impl StdUnorderedMap {
        /// Gets the maximum load factor, corresponding to the C++ `const` method.
        ///
        /// ### Incompatibilities
        ///
        /// The `overload!` macro can't distinguish between `&self` and `&mut self`
        /// receivers. These overloads also differ in their arguments and return types.
        ///
        /// ### Workarounds
        ///
        /// Use associated functions with explicit `&StdUnorderedMap` and
        /// `&mut StdUnorderedMap` parameters, as in `src/stdcpp/unordered_map.rs`.
        fn max_load_factor(&self) -> f32 {
            todo!()
        }

        /// Set the maximum load factor, a non-`const` method in C++.
        fn max_load_factor(&mut self, _ml: f32) {
            todo!()
        }
    }
}

// Accessors: `contains(...)`
overload! {
    impl StdUnorderedMap {
        /// Check whether the map contains `key`.
        fn contains(&self, _key: &K) -> bool {
            todo!()
        }

        /// Checks whether the map contains a key equal to `key`, even if it has a
        /// different type from the map's key type.
        ///
        /// ### Incompatibilities
        ///
        /// The `overload!` macro removes generic parameters, so the generated code
        /// cannot use `Q`.
        ///
        /// ### Limitations
        ///
        /// C++ only allows this overload when both `Hash` and `KeyEqual` support
        /// transparent lookup through `is_transparent`. Rust has no direct equivalent.
        ///
        /// ### Workaround
        ///
        /// Use `HashMap::get` with the `K: Borrow<Q>` trait bound instead.
        fn contains<Q>(&self, _key: &Q) -> bool {
            todo!()
        }
    }
}

pub fn main() {}
