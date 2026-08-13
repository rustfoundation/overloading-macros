#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

struct Builder {
    parts: Vec<String>,
}

overload! {
    impl Builder {
        fn build(self) -> String { self.parts.join("") }
        fn build(self, sep: String) -> String { self.parts.join(&sep) }
    }
}

fn main() {
    let b = Builder {
        parts: vec!["a".to_string(), "b".to_string(), "c".to_string()],
    };
    let result = b.build();
    assert_eq!(result, "abc");
    println!("joined: {}", result);

    let b2 = Builder {
        parts: vec!["x".to_string(), "y".to_string(), "z".to_string()],
    };
    let result2 = b2.build(", ".to_string());
    assert_eq!(result2, "x, y, z");
    println!("joined with sep: {}", result2);
}
