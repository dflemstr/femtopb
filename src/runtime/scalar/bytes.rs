use crate::{encoding, error};
use core::mem;

// Also used by `crate::item_encoding`
pub(crate) const WIRE_TYPE: encoding::WireType = encoding::WireType::LengthDelimited;

#[inline]
pub fn encode(tag: u32, value: &[u8], default: &[u8], cursor: &mut &mut [u8]) {
    if value != default {
        encode_key_value(tag, value, cursor);
    }
}

#[inline]
pub fn encode_optional(tag: u32, value: Option<&[u8]>, default: &[u8], cursor: &mut &mut [u8]) {
    if let Some(value) = value {
        if value != default {
            encode_key_value(tag, value, cursor);
        }
    }
}

crate::runtime::macros::length_delimited!('a, &'a [u8], crate::item_encoding::Bytes);

#[inline]
fn encode_key_value(tag: u32, value: &[u8], cursor: &mut &mut [u8]) {
    encoding::encode_key(tag, WIRE_TYPE, cursor);
    encode_single_value(value, cursor);
}

#[inline]
fn encode_single_value(value: &[u8], cursor: &mut &mut [u8]) {
    let len = value.len();
    encoding::encode_varint(len as u64, cursor);

    let buf = mem::take(cursor);
    let (bytes, rest) = buf.split_at_mut(len);
    *cursor = rest;
    bytes.copy_from_slice(value);
}

#[inline]
pub fn decode<'a>(
    _tag: u32,
    wire_type: encoding::WireType,
    _msg_buf: &'a [u8],
    cursor: &mut &'a [u8],
    field: &mut &'a [u8],
) -> Result<(), error::DecodeError> {
    encoding::check_wire_type(WIRE_TYPE, wire_type)?;
    *field = decode_single_value(cursor)?;
    Ok(())
}

#[inline]
pub fn decode_optional<'a>(
    _tag: u32,
    wire_type: encoding::WireType,
    _msg_buf: &'a [u8],
    cursor: &mut &'a [u8],
    field: &mut Option<&'a [u8]>,
) -> Result<(), error::DecodeError> {
    encoding::check_wire_type(WIRE_TYPE, wire_type)?;
    *field = Some(decode_single_value(cursor)?);
    Ok(())
}

// Also used by `crate::item_encoding`
pub(crate) fn decode_single_value<'a>(
    cursor: &mut &'a [u8],
) -> Result<&'a [u8], error::DecodeError> {
    let len = encoding::decode_varint(cursor)?;
    let len = usize::try_from(len).map_err(|_| error::DecodeError::LengthTooLargeForPlatform(len))?;
    if cursor.len() >= len {
        let (bytes, rest) = cursor.split_at(len);
        *cursor = rest;
        Ok(bytes)
    } else {
        Err(error::DecodeError::BufferUnderflow)
    }
}

crate::runtime::macros::decode_packed_repeated!('a, &'a [u8], crate::item_encoding::Bytes);
crate::runtime::macros::trivial_clear!('a, &'a [u8], &'static [u8], crate::item_encoding::Bytes);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_single_value_rejects_length_beyond_buffer() {
        // declares length 5 but only one content byte follows
        let buf = [0x05u8, b'a'];
        let mut cursor: &[u8] = &buf;
        assert_eq!(
            decode_single_value(&mut cursor).unwrap_err(),
            error::DecodeError::BufferUnderflow
        );
    }

    // Only reachable where `usize` is narrower than `u64`; exercised by the 32-bit CI target.
    #[cfg(target_pointer_width = "32")]
    #[test]
    fn decode_single_value_rejects_length_exceeding_usize() {
        // varint encoding of 2^32, which does not fit in a 32-bit `usize`
        let buf = [0x80u8, 0x80, 0x80, 0x80, 0x10];
        let mut cursor: &[u8] = &buf;
        assert_eq!(
            decode_single_value(&mut cursor).unwrap_err(),
            error::DecodeError::LengthTooLargeForPlatform(0x1_0000_0000)
        );
    }
}
