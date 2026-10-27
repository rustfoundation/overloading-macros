//! Demonstrate incompatible Rust translations of C++ `std::vector` overloads.
//! Expected compiler output is in `stdcpp-vector.stderr`; working overloads are in
//! `src/stdcpp/vector.rs`. This example uses Rust-only stand-ins because the crate's
//! `cpp!` build scans the library, not separately compiled examples.
//!
//! ### Incompatibilities
//!
//! Setting the element alias `T` to `c_size_t` makes the one-element initializer-list
//! signature identical to the count constructor, and the two-element signature
//! identical to the fill constructor. The generated argument-trait implementations
//! overlap. Explicit argument casts cannot distinguish identical types.
//! This collision comes from translating initializer lists into separate arguments:
//! C++ has a distinct `initializer_list<T>` parameter and braced-initialization syntax.
//!
//! The macro also rejects `front(&self)` and `front(&mut self)` in one overload set.
//! Its single dispatch method must have one receiver kind, so it cannot directly
//! represent the const and non-const C++ accessor pair.
//!
//! ### Macro ergonomics
//!
//! FIXME: The `(usize, usize)` conflict diagnostic points at the overload block
//! and displays the one-argument constructor instead of the conflicting two-argument
//! constructors. Improve the macro's generated spans so the error identifies the
//! count-and-value and two-element initializer-list overloads.
//!
//! ### Workarounds
//!
//! Give initializer-list input a distinct wrapper type, or use separate constructor
//! names for element lists and counts. Accepting slices instead is another API option,
//! but the existing slice bridge invokes the iterator-pair constructor and would not
//! demonstrate the initializer-list constructor.
//!
//! Use separate `front` and `front_mut` methods for the accessor pair. An alternative
//! worth testing is associated-function overloads with explicit `&Vector` and
//! `&mut Vector` arguments instead of receivers. A working borrowed accessor must
//! tie the returned reference's lifetime to the vector and prevent invalidating
//! mutations while that reference is borrowed.
//!
//! C++ signatures: <https://eel.is/c++draft/vector.overview>.

#![feature(splat, tuple_trait, c_size_t)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;
use std::ffi::c_size_t;

type T = c_size_t;

#[expect(dead_code, reason = "Incompatible overloads cannot be called")]
struct StdVectorT(Vec<T>);

overload! {
    impl StdVectorT {
        fn new(_count: c_size_t) -> StdVectorT {
            todo!()
        }

        fn new(_count: c_size_t, _value: T) -> StdVectorT {
            todo!()
        }

        fn new(_value: T) -> StdVectorT {
            todo!()
        }

        fn new(_a: T, _b: T) -> StdVectorT {
            todo!()
        }
    }
}

#[expect(dead_code, reason = "Incompatible overloads cannot be called")]
struct StdVectorFront(Vec<std::ffi::c_int>);

overload! {
    impl StdVectorFront {
        fn front(&self) -> Option<&std::ffi::c_int> {
            self.0.first()
        }

        fn front(&mut self) -> Option<&mut std::ffi::c_int> {
            self.0.first_mut()
        }
    }
}

fn main() {}
