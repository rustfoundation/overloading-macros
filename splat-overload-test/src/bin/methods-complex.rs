#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

struct Validator {
    threshold: i32,
}

overload! {
    impl Validator {
        fn check(&self) -> bool { self.threshold > 0 }
        fn check(&self, x: i32) -> bool {
            if x < 0 {
                return false;
            }
            x >= self.threshold
        }
        fn check(&self, x: i32, y: i32) -> Vec<bool> {
            vec![x >= self.threshold, y >= self.threshold]
        }
    }
}

fn main() {
    let v = Validator { threshold: 10 };

    assert!(v.check());
    println!("zero-arg check: {}", v.check());

    assert_eq!(v.check(-5i32), false);
    assert_eq!(v.check(15i32), true);
    println!("one-arg checks passed");

    let results = v.check(5i32, 20i32);
    assert_eq!(results, vec![false, true]);
    println!("two-arg check: {:?}", results);
}
