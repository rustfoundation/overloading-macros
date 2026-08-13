#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

struct Counter {
    value: i32,
}

overload! {
    impl Counter {
        fn add(&mut self, x: i32) { self.value += x; }
        fn add(&mut self, x: i32, y: i32) { self.value += x + y; }
    }
}

fn main() {
    let mut counter = Counter { value: 0 };

    counter.add(5i32);
    assert_eq!(counter.value, 5);
    println!("after add(5): {}", counter.value);

    counter.add(3i32, 4i32);
    assert_eq!(counter.value, 12);
    println!("after add(3, 4): {}", counter.value);
}
