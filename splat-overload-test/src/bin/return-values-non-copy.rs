#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

overload! {
    fn foo(x: String) -> String { x }
    fn foo(x: String, y: String) -> String { format!("{}{}", x, y) }
}

fn main() {
    let a = foo("We love Rust!".to_string());
    assert_eq!(a, "We love Rust!");
    println!("single string result: {}", a);

    let b = foo("We love ".to_string(), "Rust!".to_string());
    assert_eq!(b, "We love Rust!");
    println!("concatenated result: {}", b);
}
