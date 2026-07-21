# Rust Function Overloading Macros

This repository contains *experimental* macros and other code for function overloading in Rust.

Some of this code requires a recent nightly Rust compiler.

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
