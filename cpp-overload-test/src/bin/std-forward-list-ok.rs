#![feature(splat, tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use cpp::cpp;
use splat_overload::overload;
use std::ffi::{c_int, c_void};

// C++ header includes
cpp! {{
    #include <forward_list>
}}

/// A wrapper struct to hold the returned C++ pointer.
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

    fn with_capacity(capacity: usize) -> StdForwardList {
        let list = unsafe {
            cpp!([capacity as "size_t"] -> *mut c_void as "std::forward_list<int>*" {
                return new std::forward_list<int>(capacity);
            })
        };
        StdForwardList(list)
    }

    fn repeat_with(value: c_int, capacity: usize) -> StdForwardList {
        let list = unsafe {
            cpp!([value as "int", capacity as "size_t"] -> *mut c_void as "std::forward_list<int>*" {
                return new std::forward_list<int>(value, capacity);
            })
        };
        StdForwardList(list)
    }

    fn const_lvalue_copy(other: &StdForwardList) -> StdForwardList {
        let other = other.0 as *const c_void;
        let list = unsafe {
            cpp!([other as "const std::forward_list<int>*"] -> *mut c_void as "std::forward_list<int>*" {
                const std::forward_list<int> other_ = *other;
                return new std::forward_list<int>(other_);
            })
        };
        StdForwardList(list)
    }

    fn rvalue_copy(other: &mut StdForwardList) -> StdForwardList {
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

        /// Construct a list with a given capacity.
        fn new(capacity: usize) -> StdForwardList {
            StdForwardList::with_capacity(capacity)
        }

        /// Construct a list filled with the given value.
        fn new(value: c_int, capacity: usize) -> StdForwardList {
            StdForwardList::repeat_with(value, capacity)
        }

        /// Clone a list from a shared reference.
        fn new(other: &StdForwardList) -> StdForwardList {
            StdForwardList::const_lvalue_copy(other)
        }

        /// Clone a list from a mutable reference.
        /// Some C++ types use this to extend temporary lifetimes or mutate list objects during
        /// copy construction.
        fn new(other: &mut StdForwardList) -> StdForwardList {
            StdForwardList::rvalue_copy(other)
        }

        // TODO:
        // iterator
        // range
        // initializer_list
    }
}

fn main() {
    let mut default_list = StdForwardList::new();
    let _with_capacity = StdForwardList::new(10);
    let _repeat_with = StdForwardList::new(42, 100);
    let _ref_clone = StdForwardList::new(&default_list);
    let _mut_clone = StdForwardList::new(&mut default_list);
}
