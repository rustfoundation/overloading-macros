# Rust Function Overloading Macros

This repository is a [Rust Foundation *experiment*](https://rustfoundation.org/media/experimenting-with-function-overloading-in-rust-why-it-matters/) in ergonomic function overloading in Rust.

The experiment uses macros to improve the ergonomics of the [`splat` Rust language experiment](https://github.com/rust-lang/rust/issues/153629).
Technical details for the current stage of the experiment can be found [on the Inside Rust blog](https://blog.rust-lang.org/inside-rust/2026/08/19/overloading-experiment/).

Most of this code requires a recent nightly Rust compiler.

## Quick Setup

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
