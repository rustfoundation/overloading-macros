#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]

use splat_overload::overload;

overload! {
    fn calculate(a: i32, b: i32) {
        println!("sum: {}", a + b);
    }
    fn calculate(a: f64, b: f64, c: f64) {
        println!("average: {}", (a + b + c) / 3.0);
    }
    fn calculate(x: i32, y: i32, z: i32, w: i32) {
        println!("product: {}", x * y * z * w);
    }
    fn calculate(a: f64, b: f64, c: f64, d: f64, e: f64) {
        println!("max would need std: {} {} {} {} {}", a, b, c, d, e);
    }
}

fn main() {
    calculate(10, 20);
    calculate(1.0, 2.0, 3.0);
    calculate(2, 3, 4, 5);
    calculate(1.0, 2.0, 3.0, 4.0, 5.0);
}
