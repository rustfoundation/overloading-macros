#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

struct Calculator;

overload! {
    impl Calculator {
        fn compute(&self, x: i32, y: i32) { println!("sum: {}", x + y); }
        fn compute(&self, x: f64, y: f64, z: f64) { println!("average: {}", (x + y + z) / 3.0); }
    }
}

fn main() {
    let calc = Calculator;
    calc.compute(10i32, 20i32);
    calc.compute(1.0f64, 2.0f64, 3.0f64);
}
