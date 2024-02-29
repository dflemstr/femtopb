use crate::{encoding, error, unknown_fields};

pub fn decode<'a>(
    known_tags: &'static [u32],
    _wire_type: encoding::WireType,
    msg_buf: &'a [u8],
    _remaining: &mut &'a [u8],
    field: &mut unknown_fields::UnknownFields<'a>,
) -> Result<(), error::DecodeError> {
    if field.is_unpopulated() {
        *field = unknown_fields::UnknownFields::from_msg_buf(known_tags, msg_buf, true);
    }
    Ok(())
}
