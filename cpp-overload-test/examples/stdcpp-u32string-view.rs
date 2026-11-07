//! Test C++ [std::u32string_view][u32string_view] overloads that can't be represented in Rust, and record the macro diagnostics.
//! Error output is in [stdcpp-u32string-view.stderr].
//! Working overloads are in [u32string_view.rs].
//!
//! [u32string_view]: https://en.cppreference.com/cpp/string/basic_string_view
//! [stdcpp-u32string-view.stderr]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/examples/stdcpp-u32string-view.stderr
//! [u32string_view.rs]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/src/stdcpp/u32string_view.rs
//!
//! The `cpp!` macro doesn't work here, because of technical limitations it can only run one build
//! per crate. (Examples and binaries are built separately from `lib.rs`.)
//!
//! ### No receiver overloads
//!
//! The other C++ examples fail on accessor and modifier overloads, such as `begin(&self)` and
//! `begin(&mut self)`. `std::u32string_view` has no such overloads. Every accessor (`operator[]`,
//! `at`, `front`, `back`, `data`, `size`, `begin`, `end`, ...) is `const`, and the few modifiers
//! (`remove_prefix`, `remove_suffix`, `swap`) each have a single overload, so there is no
//! `&self` against `&mut self` overload set to record here. The overloads that do fail are the
//! constructor templates below.
//!
//! ### Inexpressible Overloads
//!
//! Some C++ features have no equivalent in Rust, so their overloads can't be wrapped at all:
//! - The C++23 `basic_string_view(std::nullptr_t) = delete` constructor. It exists only to make
//!   `nullptr` a compile error, and a deleted overload has no Rust equivalent.
//! - The negative constraints that choose between the templated constructors. Rust trait bounds
//!   can say what a type must implement, but not what it must *not* implement. In C++:
//!   - The iterator pair constructor `(It first, End last)` requires a `contiguous_iterator` and
//!     a `sized_sentinel_for`, and also that `End` is *not* convertible to `std::size_t`. That last
//!     rule is what stops it from taking over the pointer and count constructor
//!     `(const CharT* s, size_type count)`. Without it, the two have the same signature when `It`
//!     is a pointer and `End` is `size_t`, which is the `E0119` conflicting implementations error
//!     recorded for `StdU32StringViewPair` below.
//!   - The range constructor requires a `contiguous_range` and `sized_range`, and also that `R` is
//!     *not* the same type as `basic_string_view`, and *not* convertible to `const CharT*`. The
//!     first rule stops it from taking over the copy constructor, and the second stops it from
//!     taking over the null-terminated constructor.
//!
//!   The constraints are listed on the constructor page:
//!   <https://en.cppreference.com/w/cpp/string/basic_string_view/basic_string_view>

#![feature(splat, tuple_trait, c_size_t)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;
use std::ffi::{c_size_t, c_void};

/// A wrapper struct to hold the returned C++ `std::u32string_view` pointer.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdU32StringView(*mut c_void);

/// The C++ `char32_t` type.
type CharT = u32;

// This overload set has overloads that need generic parameters, which `overload!` can't express.
overload! {
    impl StdU32StringView {
        /// Construct a view of the first `count` characters at `s`.
        ///
        /// This overload is compatible. It is here so the set has a working overload next to the
        /// failing generic ones, and it is the overload the iterator pair clashes with in
        /// `StdU32StringViewPair` below.
        ///
        /// The working wrapper takes a `&'static [CharT]` and a count instead of a raw pointer,
        /// because `overload!` drops `unsafe`, so a raw pointer overload could be called from safe
        /// code.
        fn new(_s: *const CharT, _count: c_size_t) -> StdU32StringView {
            todo!()
        }

        /// Construct a view from a C++20 iterator and sentinel pair.
        ///
        /// ### Incompatibilities
        ///
        /// In C++ this is `template<class It, class End> basic_string_view(It first, End last)`.
        /// The Rust parameter is called `Sentinel` instead, because rustc suggests unrelated standard
        /// library items for a missing type called `End`, which would make the recorded output
        /// depend on the standard library version.
        ///
        /// The `overload!` macro doesn't support generic parameters: it builds each overload from
        /// the argument names and types only, so `It` and `Sentinel` are not declared in the code it
        /// generates, and the compiler reports "cannot find type".
        ///
        /// ### Workarounds
        ///
        /// - Replace the generic types with concrete types. The working wrapper uses a slice and a
        ///   `Range<usize>` of offsets into it.
        /// - Use fake generic types, as in the `forward_list` example: `type It = *const CharT;`
        ///   and `type Sentinel = *const CharT;` compile, and the overload is then only available
        ///   for those types.
        /// - Add generic parameter and `where` clause support to the `overload!` macro.
        fn new<It, Sentinel>(_first: It, _last: Sentinel) -> StdU32StringView {
            todo!()
        }

        /// Construct a view from a C++23 contiguous range.
        ///
        /// ### Incompatibilities
        ///
        /// In C++ this is `template<class R> explicit basic_string_view(R&& r)`, which has the
        /// same problem as the iterator pair: `R` is not declared in the generated code.
        ///
        /// ### Workarounds
        ///
        /// - Replace `R` with a concrete type, such as `&'static [CharT]`. The working wrapper
        ///   does this.
        /// - Use a fake generic type: `type R = &'static [CharT];` compiles.
        /// - Add generic parameter support to the `overload!` macro. A fully generic `R` would
        ///   also need a way to exclude `&StdU32StringView`, so it doesn't overlap the copy
        ///   constructor. C++ does that with a constraint that `R` is not the same type as
        ///   `basic_string_view`, which Rust can't express, see the "Inexpressible Overloads"
        ///   section in the [crate docs](crate#inexpressible-overloads).
        fn new<R>(_r: R) -> StdU32StringView {
            todo!()
        }
    }
}

/// A second wrapper struct for the second overload set. It also needs a different method name,
/// `newer`, because every `overload!` block generates a trait named after its method, and two
/// blocks in one module can't both generate `NewArgs`.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdU32StringViewPair(*mut c_void);

/// Workaround for missing generic support in the `overload!` macro: fake the iterator and
/// sentinel types with concrete types.
///
/// These are not called `It` and `Sentinel`, because that would declare the names used by the
/// generic overload above, and hide the "cannot find type" errors it records.
///
/// The sentinel is `c_size_t`, which is exactly what the C++ constraint "`End` is not convertible
/// to `std::size_t`" excludes.
type PairIt = *const CharT;
type PairSentinel = c_size_t;

// This overload set has overloads that clash when the fake generic types are `*const CharT` and
// `c_size_t`. The method is called `newer` here, to stand in for `new`.
overload! {
    impl StdU32StringViewPair {
        /// Construct a view of the first `count` characters at `s`.
        fn newer(_s: *const CharT, _count: c_size_t) -> StdU32StringViewPair {
            todo!()
        }

        /// Construct a view from a C++20 iterator and sentinel pair.
        ///
        /// ### Incompatibilities
        ///
        /// With `It` faked as `*const CharT` and `Sentinel` faked as `c_size_t`, this overload has
        /// the same argument types as `newer(s, count)`, but different semantics:
        /// - `newer(s, count)` views `count` characters starting at `s`.
        /// - `newer(first, last)` views the characters from `first` up to, but not including,
        ///   `last`.
        ///
        /// Rust can't have two overloads with the same argument types, so the macro generates two
        /// conflicting trait implementations (error `E0119`). C++ avoids the clash by excluding
        /// any `End` that is convertible to `std::size_t`, see the "Inexpressible Overloads"
        /// section in the [crate docs](crate#inexpressible-overloads).
        ///
        /// ### Workarounds
        ///
        /// - Don't fake `Sentinel` as `c_size_t`. A sentinel of `*const CharT` doesn't clash, and
        ///   it is the only sentinel type a pointer iterator needs.
        /// - Use different argument types for the two overloads. The working wrapper takes a slice
        ///   and a `Range<usize>` for the iterator pair, and a slice and a count for the pointer
        ///   overload.
        /// - Add a way to express negative constraints to the `overload!` macro.
        fn newer(_first: PairIt, _last: PairSentinel) -> StdU32StringViewPair {
            todo!()
        }
    }
}

pub fn main() {}
