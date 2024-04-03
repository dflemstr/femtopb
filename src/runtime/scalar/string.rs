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

    let buf = mem::replace(cursor, &mut []);
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
    let len = encoding::decode_varint(cursor)? as usize;
    if cursor.len() >= len {
        let (bytes, rest) = cursor.split_at(len);
        let string = str::from_utf8(bytes).map_err(error::DecodeError::InvalidUtf8)?;
        *cursor = rest;
        Ok(string)
    } else {
        Err(error::DecodeError::BufferUnderflow)
    }
}

crate::runtime::macros::decode_packed_repeated!('a, &'a str, crate::item_encoding::String);
crate::runtime::macros::trivial_clear!('a, &'a str, &'static str, crate::item_encoding::String);
