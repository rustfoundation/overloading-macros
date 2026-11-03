//! Test that C++ template specialisations for [std::hash][hash] can be successfully used from Rust.
//! There are no overloads for this class, the class is used via its first template parameter.
//!
//! [hash]: https://en.cppreference.com/cpp/utility/hash

use cpp::cpp;
use std::ffi::c_size_t;

// C++ header-like code
cpp! {{
    #include <unordered_set>
    #include <iostream>
    #include <cassert>

    // The `cpp!` macro can't handle C/C++ function type syntax, so we have to declare function
    // types here. This is not a limitation in production use.
    typedef int KeyCpp;
    typedef size_t HashFnCpp(const KeyCpp&);
}}

/// The same type as `KeyCpp`, but in Rust.
type KeyRust = std::ffi::c_int;

/// An ABI-compatible Rust function type for `HashFnCpp`.
pub type HashFnRust = extern "C" fn(&KeyRust) -> c_size_t;

#[test]
fn test_hash() {
    // Define a hash function in Rust.
    extern "C" fn hash_fn(key: &KeyRust) -> c_size_t {
        *key as c_size_t
    }
    // ### Ergonomics
    //
    // This coerces the unnamed function ZST object into a function type.
    // This coercion is required to use the function in C++, but the compiler does not suggest it.
    let hash_fn: HashFnRust = hash_fn;

    // Check the hash function works in Rust
    assert_eq!(hash_fn(&1), 1);
    assert_eq!(hash_fn(&444), 444);

    unsafe {
        // SAFETY: Rust functions are function pointers in C.
        //
        // ### Ergonomics
        //
        // It is surprising that a type with no explicit referneces or pointers in Rust converts to
        // a function pointer in C/C++.
        cpp!([hash_fn as "HashFnCpp*"] {
            // Check the has function works in C++
            assert(hash_fn(1) == 1);
            assert(hash_fn(444) == 444);

            const std::hash<KeyCpp>& hasher = (const std::hash<KeyCpp>&)(hash_fn);
            // And after type casting
            assert(hasher(1) == 1);
            assert(hasher(444) == 444);

            size_t bucket_count = 10;
            auto set = new std::unordered_set<KeyCpp>(bucket_count, hasher);
            assert(set->size() == 0);
            // The bucket count is a minimum
            assert(set->bucket_count() >= 10);
            set->insert(1);
            set->insert(5);
            set->insert(3333);
            set->insert(123456);
            assert(set->size() == 4);
            assert(set->bucket_count() >= 10);

            for (const auto& elem: *set) {
                assert(elem == 1 || elem == 5 || elem == 3333 || elem == 123456);
            }

            // And test the hasher works once it's been put in the set
            assert(set->hash_function()(1) == 1);
            assert(set->hash_function()(444) == 444);

            delete set;
        })
    }
}
