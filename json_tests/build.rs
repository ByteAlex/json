#[cfg(feature = "with-syntex")]
mod with_syntex {
    extern crate syntex;
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    pub fn main() {
        let out_dir = env::var_os("OUT_DIR").unwrap();

        for &(src, dst) in &[
            ("tests/test.rs.in", "test.rs"),
            ("benches/bench.rs.in", "bench.rs"),
        ] {
            let src = Path::new(src);
            let dst = Path::new(&out_dir).join(dst);

            let mut registry = syntex::Registry::new();

            serde_codegen::register(&mut registry);
            registry.expand("", &src, &dst).unwrap();
        }
    }
}

#[cfg(not(feature = "with-syntex"))]
mod with_syntex {
    pub fn main() {}
}

#[cfg(feature = "nightly-testing")]
mod nightly {
    extern crate skeptic;

    pub fn main() {
        skeptic::generate_doc_tests(&["../README.md"]);
    }
}

#[cfg(not(feature = "nightly-testing"))]
mod nightly {
    pub fn main() {}
}

pub fn main() {
    with_syntex::main();
    nightly::main();
}
