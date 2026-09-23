//! Test C++ overloads that can't be represented in Rust, and record the macro diagnostics.
//!
//! The `cpp!` macro doesn't work here, because of technical limitations it can only run one build
//! per crate. (Examples and binaries are built separately from `lib.rs`.)
//! To add C++ code here, use /* ... */ code comment blocks, or just skip it.
//!
//! ### Potential Resolutions
//!
//! There isn't a clear equivalent of list initializers in Rust, so if we decided the initializer
//! list overloads were inexpressible in Rust, all overloads would be compatible. (And initializer
//! lists could be supported using the slice or iterator overloads.)

#![feature(splat, tuple_trait, c_size_t)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;
use std::ffi::c_size_t;

/// A wrapper struct to hold the returned C++ pointer, with the `T` fake generic type.
/// We also fake the C++ list pointer with a Rust vector to help with type checking.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdForwardListT(Vec<T>);

/// Workaround for missing generic support in the `overload!` macro.
type T = std::ffi::c_size_t;

// This overload set has overloads which are incompatible when `T` is `c_size_t`.
overload! {
    impl StdForwardListT {
        /// Construct a list with `count` default-constructed items.
        fn new(_count: c_size_t) -> StdForwardListT {
            todo!()
        }

        /// Construct a list filled with `count` instances of the given value.
        fn new(_value: T, _count: c_size_t) -> StdForwardListT {
            todo!()
        }

        /// Construct a list from the supplied item.
        ///
        /// ### Incompatibilities
        ///
        /// This overload has the same types as `new(count)` when `T` is `c_size_t`, but different
        /// semantics:
        /// - `new(count)` fills the list with the default value of `T` repeated`count` times.
        /// - `new(value)` fills the list with one instance of `value`.
        fn new(_value: T) -> StdForwardListT {
            todo!()
        }

        /// Construct a list from the supplied 2 items.
        ///
        /// ### Incompatibilities
        ///
        /// This overload has the same types as `new(value, count)` when `T` is `c_size_t`, but different
        /// semantics:
        /// - `new(value, count)` fills the list with `count` instances of `value`.
        /// - `new(a, b)` fills the list with two values: `a` and `b`.
        fn new(_a: T, _b: T) -> StdForwardListT {
            todo!()
        }
    }
}

/// A wrapper struct to hold the returned C++ pointer, with the `U` fake generic type.
/// We also fake the C++ list pointer with a Rust vector to help with type checking.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdForwardListU(Vec<U>);

/// Workaround for missing generic support in the `overload!` macro.
type U = StdForwardListU;

// This overload set has overloads which are compatible, but have poor ergonomics,
// when `U` is `StdForwardListU` (which is possible with some boxed types).
overload! {
    impl StdForwardListU {
        /// Clone list items into a new list from a shared list reference.
        fn newer(_other: &StdForwardListU) -> StdForwardListU where U: Clone {
            todo!()
        }

        /// Move list items into a new list from a mutable list reference.
        fn newer(_other: &mut StdForwardListU) -> StdForwardListU {
            todo!()
        }

        /// Construct a list from the supplied item.
        ///
        /// ### Potential Overlap
        ///
        /// This overload has similar types to `new(&/&mut other)` when `U` is `StdForwardListU`,
        /// but different semantics:
        /// - `new(&/&mut other)` clones or moves the list items into a new list.
        /// - `new(value)` fills the list with one instance of `value`.
        ///
        /// Due to the references, this is technically compatible. But the ergonomics are likely
        /// to be poor, particularly when auto-ref and auto-deref are used.
        fn newer(_a: U) -> StdForwardListU {
            todo!()
        }
    }
}

/// A wrapper struct to hold the returned C++ pointer, with the `V` fake generic type.
/// We also fake the C++ list pointer with a Rust vector to help with type checking.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdForwardListV(Vec<V>);

/// Workaround for missing generic support in the `overload!` macro.
/// Currently this type alias fails compilation very early due to cycle checking, but with
/// generics the *possibility* of an identical impl existing is enough to make compilation fail,
/// even if it can't be implemented in practice.
//type V<'v> = &'v mut dyn Iterator<Item = V<'v>>;
type V = Box<dyn Iterator<Item = VCycleBreaker>>;
type VCycleBreaker = ();

// This overload set has overloads which are incompatible when `V` is
// `&mut dyn Iterator<Item = V>`. This is a contrived example, but trait solving would fail anyway.
overload! {
    impl StdForwardListV {
        /// Construct a list from an iterator.
        fn newest(_iter: &mut dyn Iterator<Item = V>) -> StdForwardListV {
            todo!()
        }

        /// Construct a list from the supplied items.
        ///
        /// ### Incompatibilities
        ///
        /// This overload has the same type as `new(iter)` when `V` is
        /// `&mut dyn Iterator<Item = V>`, but different semantics:
        /// - `new(iter)` constructs a list from the items in the iterator.
        /// - `new(a)` constructs a list from a single item (where the item is an iterator).
        fn newest(_a: V) -> StdForwardListV {
            todo!()
        }
    }
}

/// A wrapper struct to hold the returned C++ pointer, with the `W` fake generic type.
/// We also fake the C++ list pointer with a Rust vector to help with type checking.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdForwardListW(Vec<W>);

/// Workaround for missing generic support in the `overload!` macro.
type W = std::ffi::c_int;

// Accessors: `front(...)`
overload! {
    impl StdForwardList {
        /// ### Incompatibilities
        ///
        /// The `overload!` macro can't overload on `&self` vs `&mut self`, because it has to
        /// dispatch the overload through a single function, which can only have one receiver type.
        fn front(&self) -> Option<&T> where T: Sized {
            StdForwardList::front_const(self)
        }

        fn front(&mut self) -> Option<&mut T> where T: Sized {
            StdForwardList::front_mut(self)
        }
    }
}

pub fn main() {}
