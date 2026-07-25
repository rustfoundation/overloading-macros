#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

overload! {
    fn foo(x: i32) -> i32 { x * 2 }
    fn foo(x: f64) -> f64 { x * 2.0 }
}

fn main() {
    let a = foo(21);
    assert_eq!(a, 42);
    println!("i32 result: {}", a);

    let b = foo(1.5);
    assert_eq!(b, 3.0);
    println!("f64 result: {}", b);
}
