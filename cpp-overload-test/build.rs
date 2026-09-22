fn main() {
    cpp_build::build("src/bin/std-forward-list-ok.rs");
    // FIXME: move these into the crate root instead of examples, `cpp_build` doesn't work with
    // multiple examples.
    //cpp_build::build("src/bin/std-forward-list-fail.rs");
}
