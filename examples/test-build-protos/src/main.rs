use crate::generated::protobuf_unittest;

#[allow(dead_code)]
pub mod generated {
    pub mod protobuf_unittest;
    pub mod protobuf_unittest_import;
}

fn main() -> anyhow::Result<()> {
    let _ = std::hint::black_box(decode_panic_free(std::hint::black_box(&[])));
    Ok(())
}

#[no_panic::no_panic]
fn decode_panic_free(
    buffer: &[u8],
) -> Result<protobuf_unittest::TestAllTypes, femtopb::error::DecodeError> {
    use femtopb::Message as _;
    protobuf_unittest::TestAllTypes::decode(buffer)
}
