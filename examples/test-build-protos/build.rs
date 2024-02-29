use std::fs;

fn main() -> anyhow::Result<()> {
    let target = concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated");
    fs::create_dir_all(target)?;

    femtopb_build::compile_protos_into(
        &[concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/protos/google/protobuf/unittest.proto"
        )],
        &[concat!(env!("CARGO_MANIFEST_DIR"), "/protos")],
        target,
    )
}
