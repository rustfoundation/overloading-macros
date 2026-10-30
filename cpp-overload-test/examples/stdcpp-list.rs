//! Test C++ [std::list][list] overloads that can't be represented in Rust, and record the macro diagnostics.
//! Error output is in [stdcpp-list.stderr].
//! Working overloads are in [list.rs].
//!
//! [list]: https://en.cppreference.com/cpp/container/list
//! [stdcpp-list.stderr]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/examples/stdcpp-list.stderr
//! [list.rs]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/src/stdcpp/list.rs
//!
//! The `cpp!` macro doesn't work here, because of technical limitations it can only run one build
//! per crate. (Examples and binaries are built separately from `lib.rs`.) So the method bodies are
//! `todo!()`, this file only exists to show the compiler errors.
//!
//! The constructor clashes for `T = size_t` are the same as `std::forward_list`, see
//! `stdcpp-forward-list.rs`. This file covers the failures that are specific to `std::list`:
//! `back`, `insert` and `splice`.

#![feature(splat, tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

/// Workaround for missing generic support in the `overload!` macro.
type T = std::ffi::c_int;

/// A fake list, with a Rust vector standing in for the C++ list pointer.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdListX(Vec<T>);

// Accessors: `back(...)`
overload! {
    impl StdListX {
        /// Get the last element of the list as a constant reference.
        ///
        /// ### Incompatibilities
        ///
        /// Same problem as `front` in `stdcpp-forward-list.rs`. Everything in an overload set goes
        /// through one dispatch function, and that function has one receiver, so `&self` and
        /// `&mut self` can't share a set. The macro panics.
        ///
        /// ### Workarounds
        ///
        /// Associated functions taking `this: &StdList` and `this: &mut StdList`, the way `front`
        /// does in `forward_list.rs`. Until the macro handles generic lifetimes that means
        /// `&'static`, which is not pleasant to use.
        fn back(&self) -> Option<&T> {
            todo!()
        }

        /// Get the last element of the list as a mutable reference.
        fn back(&mut self) -> Option<&mut T> {
            todo!()
        }
    }
}

/// A fake iterator, standing in for `std::list<int>::const_iterator`.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdListIterY;

/// A fake list for the `insert` overloads.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdListY(Vec<T>);

// Modifiers: `insert(...)`
overload! {
    impl StdListY {
        /// Insert `value` before `pos`.
        fn insert(&mut self, _pos: &StdListIterY, _value: T) -> StdListIterY {
            todo!()
        }

        /// Insert one item from an initializer list before `pos`.
        ///
        /// ### Incompatibilities
        ///
        /// In C++ the braces pick between `insert(pos, {a})` and `insert(pos, value)`. Rust only
        /// has the argument types, and here they are identical for every `T`, not just `size_t`.
        /// So the two trait impls conflict (E0119). The constructors at least work when `T` is
        /// `int`. This one never does.
        ///
        /// ### Workarounds
        ///
        /// `list.rs` takes a slice. A tag type around the items, or a macro, would also work.
        /// Varargs on their own would not, since `insert(pos, a, b)` still looks exactly like
        /// `insert(pos, count, value)` when `T` is `size_t`.
        fn insert(&mut self, _pos: &StdListIterY, _a: T) -> StdListIterY {
            todo!()
        }
    }
}

/// A fake iterator for the `splice` overloads.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdListIterZ;

/// A fake list for the `splice` overloads.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdListZ(Vec<T>);

// Modifiers: `splice(...)`
overload! {
    impl StdListZ {
        /// Move all elements of `other` before `pos`, from a C++ `list&`.
        fn splice(&mut self, _pos: &StdListIterZ, _other: &mut StdListZ) {
            todo!()
        }

        /// Move all elements of `other` before `pos`, from a C++ `list&&`.
        ///
        /// ### Incompatibilities
        ///
        /// Every `splice` in C++ comes in a `list&` and a `list&&` flavour. Both empty `other`
        /// and leave it alive, which in Rust is `&mut StdList` either way. Two overloads, one
        /// argument type, conflicting impls (E0119).
        ///
        /// ### Workarounds
        ///
        /// `list.rs` just wraps one of them, they do the same thing anyway. The `&&` version
        /// could also take `other: StdList` by value. That compiles, since it is a different type,
        /// but now the call eats the list instead of emptying it, which is not what C++ does.
        fn splice(&mut self, _pos: &StdListIterZ, _other: &mut StdListZ) {
            todo!()
        }
    }
}

pub fn main() {}
