use crate::generated::protobuf_unittest;

// The modules are generated into `OUT_DIR` by `build.rs` (see the note there) and pulled in with
// `include!` rather than living in a checked-in source tree. The `#[allow(clippy::all, deprecated)]`
// on each module is the lint suppression that `femtopb-build` would otherwise emit as an inner
// attribute inside the file (`build.rs` strips it, since `include!` disallows a leading inner
// attribute).
#[allow(dead_code)]
pub mod generated {
    #[allow(clippy::all, deprecated)]
    pub mod protobuf_unittest {
        include!(concat!(env!("OUT_DIR"), "/protobuf_unittest.rs"));
    }
    #[allow(clippy::all, deprecated)]
    pub mod protobuf_unittest_import {
        include!(concat!(env!("OUT_DIR"), "/protobuf_unittest_import.rs"));
    }
}

fn main() -> anyhow::Result<()> {
    let _ = std::hint::black_box(decode_panic_free(std::hint::black_box(&[])));
    Ok(())
}

#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
fn decode_panic_free(
    buffer: &[u8],
) -> Result<protobuf_unittest::TestAllTypes<'_>, femtopb::error::DecodeError> {
    use femtopb::Message as _;
    protobuf_unittest::TestAllTypes::decode(buffer)
}
