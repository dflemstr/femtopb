use std::fs;

fn main() -> anyhow::Result<()> {
    let target = concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated");
    fs::create_dir_all(target)?;

    let mut config = femtopb_build::Config::new();
    config
        .target(target)
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
