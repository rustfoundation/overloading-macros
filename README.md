# Rust Function Overloading Macros

This repository is a [Rust Foundation *experiment*](https://rustfoundation.org/media/experimenting-with-function-overloading-in-rust-why-it-matters/) in ergonomic function overloading in Rust.

The experiment uses macros to improve the ergonomics of the [`splat` Rust language experiment](https://github.com/rust-lang/rust/issues/153629).
Technical details for the current stage of the experiment can be found [on the Inside Rust blog](https://blog.rust-lang.org/inside-rust/2026/08/19/overloading-experiment/).

Most of this code requires a recent nightly Rust compiler.

## Experimental Outcomes

Some outcomes of this experiment are summarised in the [C++ overloads in Rust](https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overloads-in-rust.md) document & compatibility table.

These outcomes are based on the current state of the experimental compiler feature and overloading macro: they are not Rust language design decisions.

## Macro Setup

Install a recent version of the nightly Rust compiler, and use it to build your project.
Add this `overloading-macros` crate as a dependency.

```sh
rustup update nightly
cd your-project-name
rustup override set nightly
cargo add overloading-macros ||  cargo add --git https://github.com/rustfoundation/overloading-macros overloading-macros
```

Then use the macro to add overloading to your functions or methods.

<!-- This example should be kept in sync with splat-overload-test/src/bin/readme-example.rs -->
```rust
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
```

PRs are welcome, particularly to:

- improve the macro's ergonomics
- document how to import overloads into other modules
- fix bugs in this experimental macro

## Macro Development Setup

```sh
rustup update nightly
git clone https://github.com/rustfoundation/overloading-macros
cd overloading-macros
rustup override set nightly
cargo build
```

## Running Examples

Run individual example binaries with:

```sh
cargo run --bin <filename>
```

For example:

```sh
cargo run --bin multiple-args
```
