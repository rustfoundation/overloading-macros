#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

struct Calculator;

overload! {
    impl Calculator {
        fn compute(x: i32) -> i32 { x * 2 }
        fn compute(x: i32, y: i32) -> i32 { x + y }
    }
}

fn main() {
    let a = Calculator::compute(21i32);
    assert_eq!(a, 42);
    println!("single arg result: {}", a);

    let b = Calculator::compute(10i32, 32i32);
    assert_eq!(b, 42);
    println!("two arg result: {}", b);
}
