#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

struct Calculator;

overload! {
    impl Calculator {
        fn compute(&self, x: i32) -> i32 { x }
        fn compute(&self, x: i32, y: i32) -> i32 { x + y }
    }
}

fn main() {
    let calc = Calculator;
    // Empty argument list
    calc.compute();
    calc.compute("wrong type");
    // Wrong number of arguments
    calc.compute(1_i32, 2_i32, 3);
    // Wrong return type
    let a: f64 = calc.compute(1_i32);
}
