//! Test that C++ overloads for `std::forward_list` can be successfully represented in Rust.
#![allow(unused_braces)]

use cpp::cpp;
use splat_overload::overload;
use std::ffi::{c_int, c_void};

// C++ header includes
cpp! {{
    #include <forward_list>
    #include <vector>
    #include <ranges>
}}

/// A wrapper struct to hold the returned C++ pointer.
///
/// ### Limitations
///
/// This struct and the impl should be fully generic over the list type.
/// FIXME: try this and see if the `overload!` macro and `splat` feature can handle it.
struct StdForwardList(*mut c_void);

// The cpp! macro doesn't work inside the overload! macro, so we extract C++ calls into separate methods.
// A mature overload feature (or a production C++ project) wouldn't need this impl block.
impl StdForwardList {
    fn default() -> StdForwardList {
        let list = unsafe {
            cpp!([] -> *mut c_void as "std::forward_list<int>*" {
                return new std::forward_list<int>();
            })
        };
        StdForwardList(list)
    }

    fn repeat_default(count: usize) -> StdForwardList {
        let list = unsafe {
            cpp!([count as "size_t"] -> *mut c_void as "std::forward_list<int>*" {
                return new std::forward_list<int>(count);
            })
        };
        StdForwardList(list)
    }

    fn repeat_with(value: c_int, count: usize) -> StdForwardList {
        let list = unsafe {
            cpp!([value as "int", count as "size_t"] -> *mut c_void as "std::forward_list<int>*" {
                return new std::forward_list<int>(value, count);
            })
        };
        StdForwardList(list)
    }

    /// ### Limitations
    ///
    /// The slice satisfies C++ `ContiguousIterator`, even though it's not a requirement for this
    /// C++ iterator overload.
    /// FIXME: allow non-contiguous iterators, if that makes sense in Rust.
    fn from_slice(items: &[c_int]) -> StdForwardList {
        let len = items.len();
        let items: *const c_int = items.as_ptr();
        let list = unsafe {
            cpp!([items as "const int*", len as "size_t"] -> *mut c_void as "std::forward_list<int>*" {
                // In C++, `items + len` increments the pointer by `len` elements of `sizeof(int)`.
                return new std::forward_list<int>(items, items + len);
            })
        };
        StdForwardList(list)
    }

    /// ### Limitations
    ///
    /// `std::from_range` is used as a tag to disambiguate this constructor, it is part of the C++
    /// standard library API. The Rust overload needs no tag, so it is more ergonomic.
    ///
    /// Rust does not allow `impl Iterator` in this position, so we use generics instead.
    ///
    /// This implementation builds a Rust `Vec` and C++ `std::vector` from the iterator for simplicity.
    /// A production implementation could use a Rust-to-C++ iterator-to-range adapter.
    fn from_iter(iter: &mut dyn Iterator<Item = c_int>) -> StdForwardList {
        let items: Vec<c_int> = iter.collect();
        let len = items.len();
        let items: *const c_int = items.as_ptr();
        let list = unsafe {
            cpp!([items as "const int*", len as "size_t"] -> *mut c_void as "std::forward_list<int>*" {
                // Use the legacy iterator constructor for `vector`.
                // In C++, `items + len` increments the pointer by `len` elements of `sizeof(int)`.
                std::vector<int> vec = std::vector<int>(items, items + len);
                // `vector` is a range, so we can use it to construct a `forward_list`.
                #ifdef __cpp_lib_containers_ranges
                return new std::forward_list<int>(std::from_range, vec);
                #else
                #error "std::from_range is not available, try using `-std=c++23` or a newer compiler"
                #endif
            })
        };
        StdForwardList(list)
    }

    fn copy_from(other: &StdForwardList) -> StdForwardList {
        let other = other.0 as *const c_void;
        let list = unsafe {
            cpp!([other as "const std::forward_list<int>*"] -> *mut c_void as "std::forward_list<int>*" {
                const std::forward_list<int> other_ = *other;
                return new std::forward_list<int>(other_);
            })
        };
        StdForwardList(list)
    }

    fn move_from(other: &mut StdForwardList) -> StdForwardList {
        let other: *mut c_void = other.0;
        let list = unsafe {
            cpp!([other as "std::forward_list<int>*"] -> *mut c_void as "std::forward_list<int>*" {
                std::forward_list<int> other_ = std::move(*other);
                return new std::forward_list<int>(other_);
            })
        };
        StdForwardList(list)
    }
}

// We ignore constructors that only differ by an allocator argument, because they're not
// interesting overloads to test.
overload! {
    impl StdForwardList {
        /// Construct an empty list.
        fn new() -> StdForwardList {
            StdForwardList::default()
        }

        /// Construct a list with `count` default-constructed items.
        fn new(count: usize) -> StdForwardList {
            StdForwardList::repeat_default(count)
        }

        /// Construct a list filled with `count` instances of the given value.
        fn new(value: c_int, count: usize) -> StdForwardList {
            StdForwardList::repeat_with(value, count)
        }

        /// Construct a list by copying items from a slice.
        fn new(items: &[c_int]) -> StdForwardList {
            StdForwardList::from_slice(items)
        }

        /// Construct a list from an iterator.
        ///
        /// ### Limitations
        ///
        /// `&mut dyn Iterator<T>` (and `&mut Iterator<T>`) can never be the same type as `&[T]`,
        /// but passing an owned `impl Iterator<T>` here could overlap with the `&[T]` overload.
        ///
        /// The `overload!` macro doesn't support generics in this position yet, so we use
        /// `dyn Trait` instead.
        /// FIXME: preserve function item-level generics in the macro.
        fn new(iter: &mut dyn Iterator<Item = c_int>) -> StdForwardList {
            StdForwardList::from_iter(iter)
        }

        /// Clone list items into a new list from a shared list reference.
        fn new(other: &StdForwardList) -> StdForwardList {
            StdForwardList::copy_from(other)
        }

        /// Move list items into a new list from a mutable list reference.
        fn new(other: &mut StdForwardList) -> StdForwardList {
            StdForwardList::move_from(other)
        }

        // TODO:
        // initializer_list
    }
}

pub fn test_forward_list() {
    let mut default_list = StdForwardList::new();
    let _with_capacity = StdForwardList::new(10);
    let _repeat_with = StdForwardList::new(42, 100);
    let _ref_clone = StdForwardList::new(&default_list);
    let _mut_clone = StdForwardList::new(&mut default_list);
}
