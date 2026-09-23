//! Test that C++ overloads for `std::forward_list` can be successfully represented in Rust.

#![allow(unused_braces)]

use cpp::cpp;
use splat_overload::overload;
use std::ffi::{c_size_t, c_void};
use std::marker::PhantomData;

// C++ header includes
cpp! {{
    #include <forward_list>
    #include <vector>
    #include <ranges>
    #include <cstddef>
}}

/// A wrapper struct to hold the returned C++ pointer.
///
/// ### Limitations
///
/// This struct and the impl should be fully generic over the list type.
/// FIXME: make the `overload!` macro support generics.
struct StdForwardList(*mut c_void, PhantomData<T>);

/// Workaround for missing generic support in the `overload!` macro.
type T = std::ffi::c_int;

// The cpp! macro doesn't work inside the overload! macro, so we extract C++ calls into separate methods.
// A mature overload feature (or a production C++ project) wouldn't need this impl block.
impl StdForwardList {
    fn default() -> StdForwardList {
        let list = unsafe {
            cpp!([] -> *mut c_void as "std::forward_list<int>*" {
                return new std::forward_list<int>();
            })
        };
        StdForwardList(list, PhantomData)
    }

    fn repeat_default(count: c_size_t) -> StdForwardList {
        let list = unsafe {
            cpp!([count as "size_t"] -> *mut c_void as "std::forward_list<int>*" {
                return new std::forward_list<int>(count);
            })
        };
        StdForwardList(list, PhantomData)
    }

    fn repeat_with(value: T, count: c_size_t) -> StdForwardList {
        let list = unsafe {
            cpp!([value as "int", count as "size_t"] -> *mut c_void as "std::forward_list<int>*" {
                return new std::forward_list<int>(value, count);
            })
        };
        StdForwardList(list, PhantomData)
    }

    /// ### Limitations
    ///
    /// The slice satisfies C++ `ContiguousIterator`, even though it's not a requirement for this
    /// C++ iterator overload.
    /// FIXME: allow non-contiguous iterators, if that makes sense in Rust.
    fn from_slice(items: &[T]) -> StdForwardList {
        let len = items.len();
        let items: *const T = items.as_ptr();
        let list = unsafe {
            cpp!([items as "const int*", len as "size_t"] -> *mut c_void as "std::forward_list<int>*" {
                // In C++, `items + len` increments the pointer by `len` elements of `sizeof(int)`.
                return new std::forward_list<int>(items, items + len);
            })
        };
        StdForwardList(list, PhantomData)
    }

    /// ### Limitations
    ///
    /// `std::from_range` is used as a tag to disambiguate this constructor, it is part of the C++
    /// standard library API. The Rust overload requires a cast to `dyn Iterator` instead, but this
    /// is likely just a macro limitation.
    ///
    /// This implementation builds a Rust `Vec` and C++ `std::vector` from the iterator for simplicity.
    /// A production implementation could use a Rust-to-C++ iterator-to-range adapter.
    fn from_iter(iter: &mut dyn Iterator<Item = T>) -> StdForwardList {
        let items: Vec<T> = iter.collect();
        let len = items.len();
        let items: *const T = items.as_ptr();
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
        StdForwardList(list, PhantomData)
    }

    fn copy_from(other: &StdForwardList) -> StdForwardList {
        let other = other.0 as *const c_void;
        let list = unsafe {
            cpp!([other as "const std::forward_list<int>*"] -> *mut c_void as "std::forward_list<int>*" {
                const std::forward_list<int> other_ = *other;
                return new std::forward_list<int>(other_);
            })
        };
        StdForwardList(list, PhantomData)
    }

    fn move_from(other: &mut StdForwardList) -> StdForwardList {
        let other: *mut c_void = other.0;
        let list = unsafe {
            cpp!([other as "std::forward_list<int>*"] -> *mut c_void as "std::forward_list<int>*" {
                std::forward_list<int> other_ = std::move(*other);
                return new std::forward_list<int>(other_);
            })
        };
        StdForwardList(list, PhantomData)
    }

    /// ### Limitations
    ///
    /// These are just examples, a production implementation would support any number of arguments.    
    fn from_initializer_list_1(a: T) -> StdForwardList {
        let list = unsafe {
            cpp!([a as "int"] -> *mut c_void as "std::forward_list<int>*" {
                return new std::forward_list<int>{ a };
            })
        };
        StdForwardList(list, PhantomData)
    }

    fn from_initializer_list_2(a: T, b: T) -> StdForwardList {
        let list = unsafe {
            cpp!([a as "int", b as "int"] -> *mut c_void as "std::forward_list<int>*" {
                return new std::forward_list<int>{ a, b };
            })
        };
        StdForwardList(list, PhantomData)
    }

    fn from_initializer_list_3(a: T, b: T, c: T) -> StdForwardList {
        let list = unsafe {
            cpp!([a as "int", b as "int", c as "int"] -> *mut c_void as "std::forward_list<int>*" {
                return new std::forward_list<int>{ a, b, c };
            })
        };
        StdForwardList(list, PhantomData)
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
        fn new(count: c_size_t) -> StdForwardList {
            StdForwardList::repeat_default(count)
        }

        /// Construct a list filled with `count` instances of the given value.
        fn new(value: T, count: c_size_t) -> StdForwardList {
            StdForwardList::repeat_with(value, count)
        }

        /// Construct a list by copying items from a slice.
        fn new(items: &[T]) -> StdForwardList {
            StdForwardList::from_slice(items)
        }

        /// Construct a list from an iterator.
        ///
        /// ### Limitations
        ///
        /// `&mut dyn Iterator<T>` (and `&mut Iterator<T>`) can never be the same type as `&[T]`,
        /// but passing an `impl Iterator<T>` (or `&impl ...`) here could overlap with the `&[T]`
        /// overload.
        ///
        /// The `overload!` macro doesn't support generics in this position yet, so we use
        /// `dyn Trait` instead.
        /// FIXME: make the macro support generic iterators, or handle `impl Iterator` correctly.
        fn new(iter: &mut dyn Iterator<Item = T>) -> StdForwardList {
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

        /// Construct a list from the supplied items.
        ///
        /// ### Limitations
        ///
        /// This is just an example, a production implementation would support any number of arguments.
        /// Rust doesn't have a language equivalent to C++'s `initializer_list`, and it doesn't
        /// have variadic tuples, so we can't overload on the tuple trait itself.
        ///
        /// The following initializer argument counts clash with other overloads:
        /// - 0: the default no-argument constructor, but this is acceptable, because the returned
        ///   value is the empty list in both cases.
        /// - 1: the `count` default-value repetition constructor, if `T` is `size_t`
        ///   - most constructors take 1 argument, so there could be other clashes in unusual
        ///     circumstances.
        /// - 2: the `value` repetition constructor, if `T` is `size_t`
        /// fn new(_a: T, _b: T, _c: T) -> StdForwardList {
        fn new(a: T) -> StdForwardList {
            StdForwardList::from_initializer_list_1(a)
        }

        fn new(a: T, b: T) -> StdForwardList {
            StdForwardList::from_initializer_list_2(a, b)
        }

        fn new(a: T, b: T, c: T) -> StdForwardList {
            StdForwardList::from_initializer_list_3(a, b, c)
        }
    }
}

pub fn test_forward_list() {
    let mut default_list = StdForwardList::new();
    let _repeat_default = StdForwardList::new(10);
    let _repeat_with = StdForwardList::new(42, 100);

    // ### Limitations
    //
    // The `as_slice` call is required to match the overload, an array doesn't automatically coerce.
    // FIXME: maybe add a const generic overload for arrays.
    let _from_slice = StdForwardList::new([1, 2, 3].as_slice());

    // ### Limitations
    //
    // The cast is required to match the overload, an iterator doesn't automatically coerce.
    // It would be more ergonomic for users to collect the iterator themselves, then use the slice
    // overload, or create a Rust/C++ iterator-to-range adapter.
    let mut iter: std::array::IntoIter<T, 3> = [1 as T, 2, 3].into_iter();
    let _from_iter = StdForwardList::new(&mut iter as &mut dyn Iterator<Item = _>);

    let _ref_clone = StdForwardList::new(&default_list);
    let _mut_clone = StdForwardList::new(&mut default_list);
    let _from_initializer_list = StdForwardList::new(1);
    let _from_initializer_list = StdForwardList::new(1, 2);
    let _from_initializer_list = StdForwardList::new(1, 2, 3);
}
