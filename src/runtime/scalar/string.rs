use crate::encoding;
use crate::error;
use core::{mem, str};

// Also used by `crate::item_encoding`
pub(crate) const WIRE_TYPE: encoding::WireType = encoding::WireType::LengthDelimited;

#[inline]
pub fn encode(tag: u32, value: &str, default: &str, cursor: &mut &mut [u8]) {
    if value != default {
        encode_key_value(tag, value, cursor);
    }
}

#[inline]
pub fn encode_optional(tag: u32, value: Option<&str>, default: &str, cursor: &mut &mut [u8]) {
    if let Some(value) = value {
        if value != default {
            encode_key_value(tag, value, cursor);
        }
    }
}

crate::runtime::macros::length_delimited!('a, &'a str, crate::item_encoding::String);

#[inline]
fn encode_key_value(tag: u32, value: &str, cursor: &mut &mut [u8]) {
    encoding::encode_key(tag, encoding::WireType::LengthDelimited, cursor);
    encode_single_value(value, cursor);
}

#[inline]
fn encode_single_value(value: &str, cursor: &mut &mut [u8]) {
    let len = value.len();
    encoding::encode_varint(len as u64, cursor);

    let buf = mem::take(cursor);
    let (bytes, rest) = buf.split_at_mut(len);
    *cursor = rest;
    bytes.copy_from_slice(value.as_bytes());
}

#[inline]
pub fn decode<'a>(
    _tag: u32,
    wire_type: encoding::WireType,
    _msg_buf: &'a [u8],
    cursor: &mut &'a [u8],
    field: &mut &'a str,
) -> Result<(), error::DecodeError> {
    encoding::check_wire_type(encoding::WireType::LengthDelimited, wire_type)?;
    *field = decode_single_value(cursor)?;
    Ok(())
}

#[inline]
pub fn decode_optional<'a>(
    _tag: u32,
    wire_type: encoding::WireType,
    _msg_buf: &'a [u8],
    cursor: &mut &'a [u8],
    field: &mut Option<&'a str>,
) -> Result<(), error::DecodeError> {
    encoding::check_wire_type(encoding::WireType::LengthDelimited, wire_type)?;
    *field = Some(decode_single_value(cursor)?);
    Ok(())
}

// Also used by `crate::item_encoding`
pub(crate) fn decode_single_value<'a>(
    cursor: &mut &'a [u8],
) -> Result<&'a str, error::DecodeError> {
    let len = encoding::decode_varint(cursor)?;
    let len = usize::try_from(len).map_err(|_| error::DecodeError::LengthTooLargeForPlatform(len))?;
    if cursor.len() >= len {
        let (bytes, rest) = cursor.split_at(len);
        let string = str::from_utf8(bytes).map_err(|e| error::DecodeError::InvalidUtf8 {
            valid_up_to: e.valid_up_to(),
            error_len: e.error_len(),
        })?;
        *cursor = rest;
        Ok(string)
    } else {
        Err(error::DecodeError::BufferUnderflow)
    }
}

crate::runtime::macros::decode_packed_repeated!('a, &'a str, crate::item_encoding::String);
crate::runtime::macros::trivial_clear!('a, &'a str, &'static str, crate::item_encoding::String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_single_value_rejects_invalid_utf8_with_error_len() {
        // length 2, then 0xFF 0xFF: 0xFF is an invalid leading byte, so the UTF-8 error reports
        // `valid_up_to == 0` and `error_len == Some(1)`.
        let buf = [0x02u8, 0xFF, 0xFF];
        let mut cursor: &[u8] = &buf;
        match decode_single_value(&mut cursor).unwrap_err() {
            error::DecodeError::InvalidUtf8 {
                valid_up_to,
                error_len,
            } => {
                assert_eq!(valid_up_to, 0);
                assert_eq!(error_len, Some(1));
            }
            other => panic!("expected InvalidUtf8, got {other:?}"),
        }
    }

    #[test]
    fn decode_single_value_rejects_invalid_utf8_without_error_len() {
        // length 1, then 0xE2: the start of a 3-byte sequence that is truncated, so the UTF-8
        // error reports `error_len == None`.
        let buf = [0x01u8, 0xE2];
        let mut cursor: &[u8] = &buf;
        match decode_single_value(&mut cursor).unwrap_err() {
            error::DecodeError::InvalidUtf8 {
                valid_up_to,
                error_len,
            } => {
                assert_eq!(valid_up_to, 0);
                assert_eq!(error_len, None);
            }
            other => panic!("expected InvalidUtf8, got {other:?}"),
        }
    }

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
