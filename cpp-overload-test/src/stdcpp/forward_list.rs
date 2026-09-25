//! Test that C++ overloads for [std::forward_list][forward_list] can be successfully represented in Rust.
//! Failing overloads are in [stdcpp-forward-list.rs], and error output is in [stdcpp-forward-list.stderr].
//!
//! [forward_list]: https://en.cppreference.com/cpp/container/forward_list
//! [stdcpp-forward-list.rs]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/examples/stdcpp-forward-list.rs
//! [stdcpp-forward-list.stderr]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/examples/stdcpp-forward-list.stderr

#![allow(unused_braces)]

use cpp::cpp;
use splat_overload::overload;
// ### Limitations
//
// We want `c_bool` here, but Rust only has `c_char`.
use std::ffi::{c_char, c_size_t, c_void};
use std::marker::PhantomData;

// C++ header includes
cpp! {{
    #include <forward_list>
    #include <vector>
    #include <ranges>
    #include <cstddef>

    // The `cpp!` macro can't handle C/C++ function type syntax, so we have to declare function
    // types here.
    typedef bool BinaryPredicate(const int&, const int&);
}}

/// A wrapper struct to hold the returned C++ pointer.
///
/// ### Limitations
///
/// This struct and the impl should be fully generic over the list type.
/// FIXME: make the `overload!` macro support generics.
#[cfg_attr(not(test), expect(dead_code, reason = "Only used in tests"))]
pub struct StdForwardList(*mut c_void, PhantomData<T>);

/// Workaround for missing generic support in the `overload!` macro.
pub type T = std::ffi::c_int;

// The cpp! macro doesn't work inside the overload! macro, so we extract C++ calls into separate methods.
// A mature overload feature (or a production C++ project) wouldn't need this impl block.
#[cfg_attr(not(test), expect(dead_code, reason = "Only used in tests"))]
impl StdForwardList {
    // Constructors: internal C++ implementations for `new(...)` overloads
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
                // ### Ergonomics
                //
                // This code is potentially confusing, because it is similar to an initializer
                // list constructor:
                // `new std::forward_list<int>{ item }`
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

    // Accessors: internal C++ implementations for `front(...)` overloads
    fn front_const(&self) -> Option<&T>
    where
        T: Sized,
    {
        let slf = self.0 as *const c_void;
        unsafe {
            cpp!([slf as "const std::forward_list<int>*"] -> Option<&T> as "const int*" {
                // This check is required to avoid C++ undefined behaviour on empty lists.
                if (slf->empty()) {
                    return nullptr;
                } else {
                    return &slf->front();
                }
            })
        }
    }

    fn front_mut(&mut self) -> Option<&mut T>
    where
        T: Sized,
    {
        let slf: *mut c_void = self.0;
        unsafe {
            cpp!([slf as "std::forward_list<int>*"] -> Option<&mut T> as "int*" {
                // This check is required to avoid C++ undefined behaviour on empty lists.
                if (slf->empty()) {
                    return nullptr;
                } else {
                    return &slf->front();
                }
            })
        }
    }

    // Transforms: internal implementations for `resize(...)`, `unique(...)`
    fn resize_default(&mut self, count: c_size_t) {
        let slf: *mut c_void = self.0;
        unsafe {
            cpp!([slf as "std::forward_list<int>*", count as "size_t"] {
                slf->resize(count);
            })
        }
    }

    fn resize_with(&mut self, count: c_size_t, value: T) {
        let slf: *mut c_void = self.0;
        unsafe {
            cpp!([slf as "std::forward_list<int>*", count as "size_t", value as "int"] {
                slf->resize(count, value);
            })
        }
    }

    fn unique_equals(&mut self) -> c_size_t {
        let slf: *mut c_void = self.0;
        unsafe {
            cpp!([slf as "std::forward_list<int>*"] -> c_size_t as "size_t" {
                return slf->unique();
            })
        }
    }

    fn unique_with(&mut self, eq_predicate: extern "C" fn(&T, &T) -> bool) -> c_size_t {
        let slf: *mut c_void = self.0;
        unsafe {
            // The `cpp!` macro can't handle C/C++ function type syntax, so we have to declare the
            // type separately above.
            cpp!([slf as "std::forward_list<int>*", eq_predicate as "BinaryPredicate*"] -> c_size_t as "size_t" {
                return slf->unique(eq_predicate);
            })
        }
    }

    // List checks (not overloaded)
    fn is_empty(&self) -> bool {
        let slf = self.0 as *const c_void;
        let empty = unsafe {
            cpp!([slf as "const std::forward_list<int>*"] -> c_char as "char" {
                // `char` is FFI-safe, but Rust `bool` is not.
                // C++ automatically promotes `bool` to `char`.
                return slf->empty();
            })
        };
        // And then we convert the `c_char` back to a valid Rust `bool`.
        empty != 0
    }
}

// Constructors: `new(...)`
//
// We ignore constructors that only differ by an allocator argument, because they're not
// interesting overloads to test.
overload! {
    impl StdForwardList {
        /// Construct an empty list.
        pub fn new() -> StdForwardList {
            StdForwardList::default()
        }

        /// Construct a list with `count` default-constructed items.
        pub fn new(count: c_size_t) -> StdForwardList {
            StdForwardList::repeat_default(count)
        }

        /// Construct a list filled with `count` instances of the given value.
        pub fn new(value: T, count: c_size_t) -> StdForwardList {
            StdForwardList::repeat_with(value, count)
        }

        /// Construct a list by copying items from a slice.
        pub fn new(items: &[T]) -> StdForwardList {
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
        pub fn new(iter: &mut dyn Iterator<Item = T>) -> StdForwardList {
            StdForwardList::from_iter(iter)
        }

        /// Clone list items into a new list from a shared list reference.
        pub fn new(other: &StdForwardList) -> StdForwardList {
            StdForwardList::copy_from(other)
        }

        /// Move list items into a new list from a mutable list reference.
        pub fn new(other: &mut StdForwardList) -> StdForwardList {
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
        pub fn new(a: T) -> StdForwardList {
            StdForwardList::from_initializer_list_1(a)
        }

        pub fn new(a: T, b: T) -> StdForwardList {
            StdForwardList::from_initializer_list_2(a, b)
        }

        pub fn new(a: T, b: T, c: T) -> StdForwardList {
            StdForwardList::from_initializer_list_3(a, b, c)
        }
    }
}

// Accessors: `front(...)`
overload! {
    impl StdForwardList {
        /// Get the first element of the list as a constant reference.
        ///
        /// ### Limitations
        ///
        /// The `overload!` macro can't overload on `&self` vs `&mut self`, because it has to
        /// dispatch the overload through a single function, which can only have one receiver type.
        ///
        /// The macro also doesn't support generic lifetimes, which are required here, because
        /// lifetime elision doesn't work with a non-self type (in the code generated by the
        /// macro).
        ///
        /// ### Workaround
        ///
        /// We use associated functions instead. Since `&StdForwardList` and `&mut StdForwardList`
        /// are different types, the function can be overloaded on those types.
        ///
        /// To avoid generic lifetimes, we require `&'static`. This makes usage very un-ergonomic.
        /// We could also take a `StdForwardList` and clone the element, but that doesn't work for
        /// `&mut`.
        ///
        /// ### Ergonomics
        ///
        /// Changing the return type from `&T` to `&mut T` based on the receiver is unusual in
        /// Rust, and is likely to cause confusion.
        ///
        /// ### Potential Resolutions
        ///
        /// Fall back to associated functions instead. Since different receivers can happen by
        /// accident, the macro error message should suggest either:
        /// - using the same receiver type, or
        /// - using associated functions with different receiver types.
        ///
        /// Or try modifying the macro to always add the `self` receiver to the list of overloaded
        /// types.
        pub fn front(this: &'static StdForwardList) -> Option<&'static T> where T: Sized {
            StdForwardList::front_const(this)
        }

        /// Get the first element of the list as a mutable reference.
        pub fn front(this: &'static mut StdForwardList) -> Option<&'static mut T> where T: Sized {
            StdForwardList::front_mut(this)
        }
    }
}

// Transforms: `resize(...)`, `unique(...)`
// We skip `sort(...)` because its argument set is almost identical to `unique(...)`.
overload! {
    impl StdForwardList {
        /// Resize the list to the given size, filling any new elements with the default value.
        pub fn resize(&mut self, count: c_size_t) {
            self.resize_default(count)
        }

        /// Resize the list to the given size, filling any new elements with the given value.
        pub fn resize(&mut self, count: c_size_t, value: T) {
            self.resize_with(count, value)
        }
    }
}

overload! {
    impl StdForwardList {
        /// Remove all consecutive duplicate elements from the list.
        /// Returns the number of elements removed.
        pub fn unique(&mut self) -> c_size_t {
            self.unique_equals()
        }

        /// Remove all consecutive duplicate elements from the list, using the given predicate to
        /// compare elements as equal.
        /// Returns the number of elements removed.
        pub fn unique(&mut self, eq_predicate: extern "C" fn(&T, &T) -> bool) -> c_size_t {
            self.unique_with(eq_predicate)
        }
    }
}

#[test]
pub fn test_forward_list() {
    // Constructors: `new(...)`
    let mut default_list = StdForwardList::new();
    assert!(default_list.is_empty());

    // ### Ergonomics
    //
    // This overload is easily confused with the single-argument "add item" overload.
    //
    // These `StdForwardList` type annotations are currently required, but inference should work.
    // FIXME: fix the macro or `splat` compiler implementation so inference works here.
    let repeat_default: StdForwardList = StdForwardList::new(10_usize);
    assert_eq!(repeat_default.front_const(), Some(&0));

    let repeat_with: StdForwardList = StdForwardList::new(42, 100);
    assert_eq!(repeat_with.front_const(), Some(&42));

    // ### Limitations
    //
    // The `as_slice` call is required to match the overload, an array doesn't automatically coerce.
    // FIXME: maybe add a const generic overload for arrays.
    let mut from_slice = StdForwardList::new([1, 2, 3].as_slice());
    assert_eq!(from_slice.front_const(), Some(&1));

    // ### Limitations
    //
    // The cast is required to match the overload, an iterator doesn't automatically coerce.
    // It would be more ergonomic for users to collect the iterator themselves, then use the slice
    // overload, or create a Rust/C++ iterator-to-range adapter.
    let mut iter: std::array::IntoIter<T, 3> = [1, 2, 3].into_iter();
    let from_iter = StdForwardList::new(&mut iter as &mut dyn Iterator<Item = _>);
    assert_eq!(from_iter.front_const(), Some(&1));

    let ref_clone = StdForwardList::new(&default_list);
    assert!(ref_clone.is_empty());
    let mut_clone = StdForwardList::new(&mut default_list);
    assert!(mut_clone.is_empty());

    let from_initializer_list: StdForwardList = StdForwardList::new(1);
    assert_eq!(from_initializer_list.front_const(), Some(&1));
    let from_initializer_list: StdForwardList = StdForwardList::new(2, 1);
    assert_eq!(from_initializer_list.front_const(), Some(&2));
    let from_initializer_list = StdForwardList::new(3, 2, 1);
    assert_eq!(from_initializer_list.front_const(), Some(&3));

    // Accessors: `front(...)`
    //
    // ### Workaround
    //
    // Leak these lists to get static references to them. This would never work in production.
    let static_ref =
        |list: StdForwardList| -> &'static StdForwardList { &*Box::leak(Box::new(list)) };
    let static_mut =
        |list: StdForwardList| -> &'static mut StdForwardList { Box::leak(Box::new(list)) };

    assert_eq!(StdForwardList::front(static_ref(ref_clone)), None);
    assert_eq!(StdForwardList::front(static_mut(mut_clone)), None);
    assert_eq!(StdForwardList::front(static_ref(repeat_default)), Some(&0));
    assert_eq!(
        StdForwardList::front(static_mut(repeat_with)),
        Some(&mut 42)
    );

    // Transforms: `resize(...)`, `unique(...)`
    from_slice.resize(0);
    assert!(from_slice.is_empty());
    let mut resized_list = StdForwardList::new();
    resized_list.resize(1, 42);
    assert_eq!(resized_list.front_const(), Some(&42));

    let mut unique_list = StdForwardList::new([1, 2, 2, 3, 3, 3, 1].as_slice());
    assert_eq!(unique_list.unique(), 3);
    assert_eq!(unique_list.front_const(), Some(&1));

    let mut unique_list = StdForwardList::new([1, 2, 2, 3, 3, 3, 1].as_slice());
    extern "C" fn predicate(a: &T, b: &T) -> bool {
        a == b
    }
    assert_eq!(unique_list.unique(predicate), 3);
    assert_eq!(unique_list.front_const(), Some(&1));
}
