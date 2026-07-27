use std::{env, fs, path::Path};

fn main() -> anyhow::Result<()> {
    // Generate into `OUT_DIR` (under `target/`, git-ignored) and `include!` the result, rather than
    // writing into a checked-in source tree. The generated code is a pure derivative of the `.proto`
    // files, so committing it only invites drift: every build overwrites it, and a build with a
    // different feature set (e.g. `defmt`, which adds `#[derive(defmt::Format)]`) produces different
    // output. Generating on demand keeps it always in sync and never dirties the working tree.
    let out_dir = env::var("OUT_DIR")?;

    let modules = ["protobuf_unittest.rs", "protobuf_unittest_import.rs"];

    let mut config = femtopb_build::Config::new();
    config
        .target(&out_dir)
        .protos(&[concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/protos/google/protobuf/unittest.proto"
        )])
        .includes(&[concat!(env!("CARGO_MANIFEST_DIR"), "/protos")]);

    if cfg!(feature = "defmt") {
        config.derive_defmt(true);
    }

    config.compile()?;

    // `femtopb-build` prefixes each module with an *inner* lint-suppression attribute
    // (`#![allow(clippy::all, deprecated)]`). That is fine for the usual `pub mod foo;`
    // (file-as-module) wiring, but `include!(...)` inside a `mod { ... }` block rejects a leading
    // inner attribute. Since this crate pulls the modules in with `include!`, strip that attribute
    // here; `main.rs` re-applies the same allow as an *outer* attribute on each module instead.
    for module in modules {
        let path = Path::new(&out_dir).join(module);
        let contents = fs::read_to_string(&path)?;
        if let Some(rest) = contents.strip_prefix("#![allow(clippy::all, deprecated)]") {
            fs::write(&path, rest.trim_start())?;
        }
    }

    Ok(())
}
