#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features, clippy::approx_constant)]

use splat_overload::overload;

overload! {
    fn foo(_x: i32, _y: f64) {}
    fn foo(_x: bool, _y: i32, _z: f64) {}
    fn foo(_a: i32, _b: f64, _c: bool, _d: u8) {}
}

fn main() {
    // Empty argument list
    foo();
    foo("one wrong type", 2.0_f64);
    // Wrong number of arguments
    foo(1_i32, 2_f64, true, 4_u8, 5);
}
