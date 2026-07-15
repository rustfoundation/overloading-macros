#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features, clippy::approx_constant)]

use splat_overload::overload;

overload! {
    fn foo(x: i32, y: f64) { 
        println!("i32: {}, f64: {}", x, y); 
    }
    fn foo(x: bool, y: i32, z: f64) { 
        println!("bool: {}, i32: {}, f64: {}", x, y, z); 
    }
    fn foo(a: i32, b: f64, c: bool, d: u8) { 
        println!("i32: {}, f64: {}, bool: {}, u8: {}", a, b, c, d); 
    }
}

fn main() {
    foo(42, 3.14);
    foo(true, 42, 3.14);
    foo(42, 3.14, true, 255);
}