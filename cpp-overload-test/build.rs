fn main() {
    // `cpp_build` doesn't work with multiple example/binary builds in the same crate, so we use
    // lib.rs submodules for successful overloads.
    cpp_build::Config::new()
        // Required for range overloads.
        .flag("-std=c++23")
        .build("src/lib.rs");
}
