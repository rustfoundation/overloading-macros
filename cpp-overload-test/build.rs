fn main() {
    // `cc` prints `rerun-if-env-changed` lines, which turns off Cargo's default rescan of the
    // package, so instruct Cargo to rerun this script whenever `src/` changes and pick up
    // `cpp!` updates.
    println!("cargo::rerun-if-changed=src");
    // `cpp_build` doesn't work with multiple example/binary builds in the same crate, so we use
    // lib.rs submodules for successful overloads.
    cpp_build::Config::new()
        // Required for range overloads.
        .flag("-std=c++23")
        // Ignore `warning: 'std::rel_ops' is deprecated: use '<=>' instead [-Wdeprecated-declarations]`
        .flag("-Wno-deprecated-declarations")
        .build("src/lib.rs");
}
