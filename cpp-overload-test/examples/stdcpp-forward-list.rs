//! Test C++ overloads that can't be represented in Rust, and record the macro diagnostics.
//!
//! The `cpp!` macro doesn't work here, because of technical limitations it can only run one build
//! per crate. (Examples and binaries are built separately from `lib.rs`.)
//! To add C++ code here, use /* ... */ code comment blocks.

fn main() {
    const _: () = panic!("There are no failing overloads for C++ `std::forward_list` yet");
}
