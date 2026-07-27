use std::env;

fn main() -> anyhow::Result<()> {
    // Generate into `OUT_DIR` (under `target/`, git-ignored) and `include!` the result, rather than
    // writing into a checked-in source tree. The generated code is a pure derivative of the `.proto`
    // files, so committing it only invites drift: every build overwrites it, and a build with a
    // different feature set (e.g. `defmt`, which adds `#[derive(defmt::Format)]`) produces different
    // output. Generating on demand keeps it always in sync and never dirties the working tree.
    let out_dir = env::var("OUT_DIR")?;

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

    config.compile()
}
