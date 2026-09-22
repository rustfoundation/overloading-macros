fn main() {
    // `cpp_build` doesn't work with multiple example/binary builds in the same crate, so we use
    // lib.rs submodules for successful overloads.
    cpp_build::build("src/lib.rs");
}
