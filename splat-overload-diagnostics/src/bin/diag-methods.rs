#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces, clippy::disallowed_names)]

use splat_overload::overload;

struct Foo;

overload! {
    impl Foo {
        fn method(&self, _x: i32) {}
        fn method(&self, _x: f64) {}
    }
}

fn main() {
    let foo = Foo;
    // Empty argument list
    foo.method();
    foo.method("wrong type");
    // Wrong number of arguments
    foo.method(1_i32, 2);
}
