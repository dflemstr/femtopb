use crate::generated::protobuf_unittest;

// The modules are generated into `OUT_DIR` by `build.rs` (see the note there) and pulled in with
// `include!` rather than living in a checked-in source tree. `femtopb-build` now puts its
// lint-suppression `#[allow(clippy::all, deprecated)]` on each generated item (not as a module inner
// attribute), so `include!` works directly with nothing to strip. The module-level allow below is
// only for the `defmt` feature: with it, each generated struct also gets `#[derive(defmt::Format)]`,
// and a foreign derive's output can only be lint-suppressed from an enclosing scope, not per-item.
#[allow(dead_code)]
#[allow(clippy::all, deprecated)]
pub mod generated {
    pub mod protobuf_unittest {
        include!(concat!(env!("OUT_DIR"), "/protobuf_unittest.rs"));
    }
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
