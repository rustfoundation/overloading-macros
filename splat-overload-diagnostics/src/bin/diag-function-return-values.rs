#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

overload! {
    fn foo(x: i32) -> i32 { x }
    fn foo(x: f64) -> f64 { x }
}

fn main() {
    // Empty argument list
    foo();
    foo("wrong type");
    // Wrong number of arguments
    foo(1_i32, 2, 3);
    // Wrong return type
    let a: i32 = foo(1.0_f64);
}
