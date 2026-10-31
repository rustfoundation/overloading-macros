//! Test C++ [std::unordered_multiset][unordered_multiset] overloads that can't be represented in Rust.
//! Working overloads are in [unordered_multiset.rs].
//!
//! [unordered_multiset]: https://en.cppreference.com/cpp/container/unordered_multiset
//! [unordered_multiset.rs]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/src/stdcpp/unordered_multiset.rs
//!
//! The `cpp!` macro doesn't work here because examples are built separately from `lib.rs`.

#![feature(splat, tuple_trait, c_size_t)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;
use std::ffi::c_void;
use std::marker::PhantomData;

#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdUnorderedMultiset(*mut c_void, PhantomData<T>);

type T = std::ffi::c_int;

// The overload! macro can't overload on &self vs &mut self
overload! {
    impl StdUnorderedMultiset {
        /// ### Incompatibilities
        ///
        /// The overload! macro can't overload on `&self` vs `&mut self`, because it has to
        /// dispatch the overload through a single function, which can only have one receiver type.
        fn begin(&self) where T: Sized {
            todo!()
        }

        fn begin(&mut self) where T: Sized {
            todo!()
        }
    }
}

pub fn main() {}
