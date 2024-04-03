use crate::encoding;
use crate::enumeration;
use crate::error;
use crate::packed;
use crate::repeated;

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encode<E>(
    tag: u32,
    value: enumeration::EnumValue<E>,
    default: Option<E>,
    cursor: &mut &mut [u8],
) where
    E: enumeration::Enumeration,
{
    let raw_default = default.unwrap_or(E::default()).encode();
    let raw_value = value.to_raw();
    if raw_value != raw_default {
        encode_key_value(tag, value, cursor);
    }
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encode_optional<E>(
    tag: u32,
    value: Option<enumeration::EnumValue<E>>,
    default: Option<E>,
    cursor: &mut &mut [u8],
) where
    E: enumeration::Enumeration,
{
    if let Some(v) = value {
        encode(tag, v, default, cursor)
    }
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encode_repeated<E>(
    tag: u32,
    values: repeated::Repeated<enumeration::EnumValue<E>, crate::item_encoding::Enum<E>>,
    cursor: &mut &mut [u8],
) where
    E: enumeration::Enumeration,
{
    for result in values {
        if let Ok(value) = result {
            encode_key_value(tag, value, cursor);
        }
    }
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encode_packed<E>(
    tag: u32,
    values: packed::Packed<enumeration::EnumValue<E>, crate::item_encoding::Enum<E>>,
    cursor: &mut &mut [u8],
) where
    E: enumeration::Enumeration,
{
    if !values.is_empty() {
        encoding::encode_key(tag, encoding::WireType::LengthDelimited, cursor);
        let len: usize = values
            .iter()
            .map(|r| encoding::encoded_len_varint(r.map(|v| v.to_raw()).unwrap_or(0) as u64))
            .sum();
        encoding::encode_varint(len as u64, cursor);

        for result in values {
            if let Ok(value) = result {
                encode_single_value(value, cursor);
            }
        }
    }
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
fn encode_key_value<E>(tag: u32, value: enumeration::EnumValue<E>, cursor: &mut &mut [u8])
where
    E: enumeration::Enumeration,
{
    encoding::encode_key(tag, encoding::WireType::Varint, cursor);
    encode_single_value(value, cursor);
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
fn encode_single_value<E>(value: enumeration::EnumValue<E>, cursor: &mut &mut [u8])
where
    E: enumeration::Enumeration,
{
    encoding::encode_varint(value.to_raw() as u64, cursor);
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encoded_len<E>(tag: u32, value: enumeration::EnumValue<E>, default: Option<E>) -> usize
where
    E: enumeration::Enumeration,
{
    let raw_default = default.unwrap_or(E::default()).encode();
    let raw_value = value.to_raw();
    if raw_value == raw_default {
        0
    } else {
        encoding::key_len(tag) + encoding::encoded_len_varint(value.to_raw() as u64)
    }
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encoded_len_optional<E>(
    tag: u32,
    value: Option<enumeration::EnumValue<E>>,
    default: Option<E>,
) -> usize
where
    E: enumeration::Enumeration,
{
    if let Some(v) = value {
        encoded_len(tag, v, default)
    } else {
        0
    }
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encoded_len_repeated<E>(
    tag: u32,
    values: repeated::Repeated<enumeration::EnumValue<E>, crate::item_encoding::Enum<E>>,
) -> usize
where
    E: enumeration::Enumeration,
{
    encoding::key_len(tag) * values.len()
        + values
            .iter()
            .map(|r| {
                r.map(|v| encoding::encoded_len_varint(v.to_raw() as u64))
                    .unwrap_or(0)
            })
            .sum::<usize>()
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encoded_len_packed<E>(
    tag: u32,
    values: packed::Packed<enumeration::EnumValue<E>, crate::item_encoding::Enum<E>>,
) -> usize
where
    E: enumeration::Enumeration,
{
    if values.is_empty() {
        0
    } else {
        let len = values
            .iter()
            .map(|r| {
                r.map(|v| encoding::encoded_len_varint(v.to_raw() as u64))
                    .unwrap_or(0)
            })
            .sum::<usize>();
        encoding::key_len(tag) + encoding::encoded_len_varint(len as u64) + len
    }
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn decode<'a, E>(
    _tag: u32,
    wire_type: encoding::WireType,
    _msg_buf: &'a [u8],
    remaining: &mut &'a [u8],
    field: &mut enumeration::EnumValue<E>,
) -> Result<(), error::DecodeError>
where
    E: enumeration::Enumeration,
{
    encoding::check_wire_type(encoding::WireType::Varint, wire_type)?;
    *field = decode_single_value(remaining)?;
    Ok(())
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn decode_optional<'a, E>(
    _tag: u32,
    wire_type: encoding::WireType,
    _msg_buf: &'a [u8],
    remaining: &mut &'a [u8],
    field: &mut Option<enumeration::EnumValue<E>>,
) -> Result<(), error::DecodeError>
where
    E: enumeration::Enumeration,
{
    encoding::check_wire_type(encoding::WireType::Varint, wire_type)?;
    *field = Some(decode_single_value(remaining)?);
    Ok(())
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub(crate) fn decode_single_value<E>(
    remaining: &mut &[u8],
) -> Result<enumeration::EnumValue<E>, error::DecodeError>
where
    E: enumeration::Enumeration,
{
    Ok(E::decode(encoding::decode_varint(remaining)? as i32))
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn decode_repeated<'a, E>(
    tag: u32,
    wire_type: encoding::WireType,
    msg_buf: &'a [u8],
    cursor: &mut &'a [u8],
    field: &mut repeated::Repeated<'a, enumeration::EnumValue<E>, crate::item_encoding::Enum<E>>,
) -> Result<(), error::DecodeError>
where
    E: enumeration::Enumeration,
{
    if field.is_unpopulated() {
        *field = repeated::Repeated::from_msg_buf(tag, msg_buf);
    }
    encoding::skip_field(wire_type, tag, cursor)?;
    Ok(())
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn decode_packed<'a, E>(
    tag: u32,
    _wire_type: encoding::WireType,
    msg_buf: &'a [u8],
    _remaining: &mut &'a [u8],
    field: &mut packed::Packed<'a, enumeration::EnumValue<E>, crate::item_encoding::Enum<E>>,
) -> Result<(), error::DecodeError>
where
    E: enumeration::Enumeration,
{
    if field.is_unpopulated() {
        *field = packed::Packed::from_msg_buf(tag, msg_buf);
    }
    Ok(())
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn clear<E>(value: &mut enumeration::EnumValue<E>)
where
    E: enumeration::Enumeration,
{
    *value = enumeration::EnumValue::Known(E::default());
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn clear_optional<E>(value: &mut Option<enumeration::EnumValue<E>>)
where
    E: enumeration::Enumeration,
{
    *value = None;
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn clear_repeated<E>(
    value: &mut repeated::Repeated<enumeration::EnumValue<E>, crate::item_encoding::Enum<E>>,
) where
    E: enumeration::Enumeration,
{
    *value = repeated::Repeated::empty();
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn clear_packed<E>(
    value: &mut packed::Packed<enumeration::EnumValue<E>, crate::item_encoding::Enum<E>>,
) where
    E: enumeration::Enumeration,
{
    *value = packed::Packed::empty();
}
