#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

overload! {
    fn foo() -> i32 { 42 }
    fn foo(x: i32) -> i32 { x }
}

fn main() {
    let a = foo();
    assert_eq!(a, 42);
    println!("zero args result: {}", a);

    let b = foo(10i32);
    assert_eq!(b, 10);
    println!("one arg result: {}", b);
}
