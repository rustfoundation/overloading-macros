//! Test C++ `std::priority_queue` overloads that can't be represented in Rust, and record the
//! macro diagnostics. Working overloads are in `cpp-overload-test/src/stdcpp/priority_queue.rs`.
//!
//! ### Incompatibilities
//!
//! `overload!` doesn't support generic type parameters, so the iterator-pair constructor and the
//! generic `emplace` fail with E0425.
//!
//! ### Workarounds
//!
//! Use concrete types, as in `priority_queue.rs`: `&mut dyn Iterator` (callers need a cast), and
//! zero or one `int` arguments for `emplace`.

#![feature(splat, tuple_trait, c_size_t)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;
use std::ffi::c_int;

#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdPriorityQueueIter(Vec<c_int>);

overload! {
    impl StdPriorityQueueIter {
        fn new() -> StdPriorityQueueIter {
            todo!()
        }

        fn new<I: Iterator<Item = c_int>>(_first: I, _last: I) -> StdPriorityQueueIter {
            todo!()
        }
    }
}

#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdPriorityQueueEmplace(Vec<c_int>);

overload! {
    impl StdPriorityQueueEmplace {
        fn emplace(&mut self) {
            todo!()
        }

        fn emplace<A: Into<c_int>>(&mut self, _a: A) {
            todo!()
        }
    }
}

pub fn main() {}
