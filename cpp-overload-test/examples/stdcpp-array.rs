//! Test C++ [std::array][array] overloads that can't be represented in Rust, and record the macro diagnostics.
//! Error output is in [stdcpp-array.stderr].
//! Working overloads are in [array.rs].
//!
//! [array]: https://en.cppreference.com/cpp/container/array
//! [stdcpp-array.stderr]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/examples/stdcpp-array.stderr
//! [array.rs]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/src/stdcpp/array.rs
//!
//! The `cpp!` macro doesn't work here, because of technical limitations it can only run one build
//! per crate. (Examples and binaries are built separately from `lib.rs`.)
//! To add C++ code here, use /* ... */ code comment blocks, or just skip it.
//!
//! `std::array`'s modifiers, `fill` and `swap`, aren't overloaded, so they have no failing
//! overloads.
//!
//! ### Inexpressible Overloads
//!
//! C++ has two ways to create an array with no arguments. `std::array<int, 3> a;` leaves the `int`
//! elements uninitialized, and `std::array<int, 3> a{};` sets them to zero. C++ chooses between
//! them using syntax, not their argument types, so Rust overloads can only represent one of them.
//!
//! C++ class template argument deduction (CTAD) lets `std::array a{1, 2, 3};` work out `N` from
//! the number of initializers. A Rust associated function like `new` belongs to one concrete type,
//! so it can't pick `N`. A free function overload set could return a different array size for each
//! argument count, but until the macro supports const generics, each size needs its own wrapper
//! type.

#![feature(splat, tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

/// Workaround for missing generic support in the `overload!` macro.
type T = std::ffi::c_int;

/// A wrapper struct to hold the C++ array, with the `T` fake generic type and a real const
/// generic size. We fake the C++ array pointer with a Rust array to help with type checking.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdArrayN<const N: usize>([T; N]);

// Constructors: `new(...)` for any array size.
//
// ### Incompatibilities
//
// In C++ the array size `N` is a non-type template parameter, which maps to a Rust const
// generic. `overload!` drops the generics on the `impl` block, and keeps only the last
// name in the type path, so the generated code refers to `StdArrayN` without its size.
//
// ### Workarounds
//
// Use a fixed size, like `StdArray3` in `array.rs`, until
// the macro supports generics.
//
// ### Ergonomics
//
// The compiler's suggestions describe the code the macro generates, not this source. It suggests
// replacing `N` with `T` and adding a second `<N>` after `StdArrayN<N>`, and neither of those
// helps.
//
// FIXME: make `overload!` keep the `impl` generics, or report a clear error when it drops them.
//
// These notes can't be doc comments on the `impl` block, because the macro only
// recognises an `impl` block when `impl` is its first token.
overload! {
    impl<const N: usize> StdArrayN<N> {
        /// Construct an array with every element value-initialized.
        fn new() -> StdArrayN<N> {
            todo!()
        }

        /// Construct an array from 1 item, and value-initialize the other elements.
        fn new(_a: T) -> StdArrayN<N> {
            todo!()
        }
    }
}

/// A wrapper struct to hold the C++ `std::array<int, 3>`, with the `T` fake generic type.
/// We fake the C++ array pointer with a Rust array to help with type checking.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdArray3([T; 3]);

// Accessors: `at(...)`
overload! {
    impl StdArray3 {
        /// Get a mutable reference to the element at `pos`.
        ///
        /// ### Incompatibilities
        ///
        /// C++ overloads `at` on whether the array is `const`, which maps to Rust `&self`
        /// and `&mut self`. `overload!` can't overload on the receiver, because it
        /// dispatches every overload through a single function, which can only have one receiver
        /// type. `front`, `back`, `data`, `begin`, `end`, `rbegin`, and `rend` fail in the same
        /// way.
        ///
        /// ### Workarounds
        ///
        /// Pass the array as an ordinary argument instead of `self`, like `get` in
        /// `array.rs`. `&StdArray3` and `&mut StdArray3` are different types, so they can be
        /// overloaded.
        fn at(&mut self, _pos: usize) -> &mut T {
            todo!()
        }

        /// Get a constant reference to the element at `pos`.
        fn at(&self, _pos: usize) -> &T {
            todo!()
        }
    }
}

// Non-member functions: `get(...)`
overload! {
    /// Get a mutable reference to element `I`.
    ///
    /// ### Incompatibilities
    ///
    /// C++ `std::get<I>(array)` takes the element index as a template argument, which maps to a
    /// Rust const generic. `overload!` drops each function's generics, so `I` doesn't exist in the
    /// generated code. Even with generics, the generated `get` function is only generic over its
    /// argument types, so a call like `get::<0>(array)` has nowhere to put the index.
    ///
    /// ### Workarounds
    ///
    /// Pass the index as a value of a zero-sized tag type, `Index<I>`, and write one overload for
    /// each index. This is similar to C++ tag dispatch, and `array.rs` uses it for `get`.
    /// A mature overloading feature should support const generic arguments instead.
    fn get<const I: usize>(array: &'static mut StdArray3) -> &'static mut T {
        &mut array.0[I]
    }

    /// Get a constant reference to element `I`.
    fn get<const I: usize>(array: &'static StdArray3) -> &'static T {
        &array.0[I]
    }
}

pub fn main() {}
