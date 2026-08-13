#![feature(splat)]
#![feature(tuple_trait)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;

struct Logger {
    prefix: String,
}

overload! {
    impl Logger {
        fn log(&self, message: String) { println!("{}: {}", self.prefix, message); }
        fn log(&self, message: String, level: String) { println!("{} [{}]: {}", self.prefix, level, message); }
    }
}

fn main() {
    let logger = Logger {
        prefix: "APP".to_string(),
    };
    logger.log("starting up".to_string());
    logger.log("something happened".to_string(), "WARN".to_string());
}
