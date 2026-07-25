#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

overload! {
    fn foo(x: i32) -> i32 {
        if x < 0 {
            return 0;
        }
        x * 2
    }
    fn foo(x: i32, y: i32) -> Vec<i32> { vec![x, y] }
    fn foo(x: i32, y: i32, z: i32) -> String { format!("{}-{}-{}", x, y, z) }
}

fn main() {
    let a = foo(-5);
    assert_eq!(a, 0);
    println!("early return result: {}", a);

    let b = foo(10);
    assert_eq!(b, 20);
    println!("normal path result: {}", b);

    let c = foo(1i32, 2);
    assert_eq!(c, vec![1, 2]);
    println!("vec result: {:?}", c);

    let d = foo(1i32, 2, 3);
    assert_eq!(d, "1-2-3");
    println!("string result: {}", d);
}
