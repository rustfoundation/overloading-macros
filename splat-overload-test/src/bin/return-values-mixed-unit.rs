#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

overload! {
    fn foo(x: i32) -> i32 { x * 2 }
    fn foo(x: f64) { println!("f64: {}", x); }
}

fn main() {
    let a = foo(21);
    assert_eq!(a, 42);
    println!("i32 result: {}", a);
    foo(2.5);
    println!("f64 call completed");
}
