//! Test c++ std::chrono::time_point
//!
//! Working overloads are in [time_point.rs]
//!
//! [time_point.rs]: ../src/stdcpp/time_point.rs

#![feature(splat, tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;
use std::ffi::c_longlong;

/// A fake Rust wrapper representing a C++ time_point.
///
/// Using vector to represent the time point because this example only tests
/// overload compatibility.
#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdTimePoint(Vec<c_longlong>);

/// Workaround for testing identical time point types.
///
/// The converting constructor can accept a time point with the same
/// duration type as the copy constructor.
type SameTimePoint = StdTimePoint;

// Constructors: new(...)
//
// When copy constructor and converting constructor receive the same
// time_point type, their argument types become identical.
//
// c++ resolves this by preferring the non-template copy constructor.
// Rust cannot represent both as separate overloads.

overload! {
	impl StdTimePoint {

	/// Copy constructor
	fn new(_other: &StdTimePoint) -> StdTimePoint {
		todo!()
	}

	/// Converting constructor
	fn new(_other: &SameTimePoint) -> StdTimePoint {
		todo!()
	}
    }
}

pub fn main() {}
