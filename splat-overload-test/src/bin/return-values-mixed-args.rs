#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

overload! {
    fn foo(x: f64, y: i32) -> i32 { (x as i32) + y }
    fn foo(x: i32) -> i32 { x * 2 }
}

fn main() {
    let a = foo(3.7, 10);
    assert_eq!(a, 13);
    println!("mixed args result: {}", a);

    let b = foo(21);
    assert_eq!(b, 42);
    println!("single arg result: {}", b);
}
