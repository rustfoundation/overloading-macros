//! This example should be kept in sync with the README.md file.

#![feature(splat, tuple_trait)]
#![allow(incomplete_features, unused_braces)]

use splat_overload::overload;

// Functions can be overloaded
overload! {
    fn show(num: i32) { println!("num: {}", num); }
    fn show(nums: Vec<i32>) { println!("nums: {:?}", nums); }
}

struct Example;

// So can methods and return values
overload! {
    impl Example {
        fn tell(&self, num: i32) -> i32 { println!("num: {}", num); return num; }
        fn tell(&self, nums: Vec<i32>) -> Vec<i32> { println!("nums: {:?}", nums); return nums; }
    }
}

fn main() {
    show(42);
    show(vec![42, 43, 44]);

    let e = Example;
    let _num = e.tell(42);
    let _nums = e.tell(vec![42, 43, 44]);
}
