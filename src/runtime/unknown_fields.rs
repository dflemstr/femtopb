use crate::{encoding, error, unknown_fields};

#[inline]
#[cold]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn decode<'a>(
    known_tags: &'static [u32],
    matched_tag: u32,
    wire_type: encoding::WireType,
    msg_buf: &'a [u8],
    remaining: &mut &'a [u8],
    field: &mut unknown_fields::UnknownFields<'a>,
) -> Result<(), error::DecodeError> {
    if field.is_unpopulated() {
        *field = unknown_fields::UnknownFields::from_msg_buf(known_tags, msg_buf, true);
    }
    encoding::skip_field(wire_type, matched_tag, remaining)
}
