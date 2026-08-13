#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces, clippy::disallowed-names)]

use splat_overload::overload;

struct Foo;

overload! {
    impl Foo {
        fn method(&self, x: i32) { println!("i32: {}", x); }
        fn method(&self, x: f64) { println!("f64: {}", x); }
    }
}

fn main() {
    let foo = Foo;
    foo.method(42i32);
    foo.method(3.1f64);
}
