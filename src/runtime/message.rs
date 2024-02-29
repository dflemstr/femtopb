use crate::repeated;
use crate::{encoding, error};
use crate::{item_encoding, message};

#[inline]
pub fn encode<'a, M>(tag: u32, value: &M, cursor: &mut &mut [u8])
where
    M: message::Message<'a>,
{
    encoding::encode_key(tag, encoding::WireType::LengthDelimited, cursor);
    encoding::encode_varint(u64::try_from(value.encoded_len()).unwrap(), cursor);
    value.encode_raw(cursor);
}

#[inline]
pub fn encode_optional<'a, M>(tag: u32, value: &Option<M>, cursor: &mut &mut [u8])
where
    M: message::Message<'a>,
{
    if let Some(m) = value.as_ref() {
        encode(tag, m, cursor)
    }
}

#[inline]
pub fn encode_repeated<'a, M>(
    tag: u32,
    values: &repeated::Repeated<'a, M, item_encoding::Message<'a, M>>,
    cursor: &mut &mut [u8],
) where
    M: message::Message<'a>,
{
    for result in values {
        if let Ok(value) = result {
            encode(tag, &value, cursor);
        }
    }
}

#[inline]
pub fn encoded_len<'a, M>(tag: u32, value: &M) -> usize
where
    M: message::Message<'a>,
{
    let len = value.encoded_len();
    encoding::key_len(tag) + encoding::encoded_len_varint(u64::try_from(len).unwrap()) + len
}

#[inline]
pub fn encoded_len_optional<'a, M>(tag: u32, value: &Option<M>) -> usize
where
    M: message::Message<'a>,
{
    if let Some(m) = value.as_ref() {
        encoded_len(tag, m)
    } else {
        0
    }
}

#[inline]
pub fn encoded_len_repeated<'a, M>(
    tag: u32,
    values: &repeated::Repeated<'a, M, item_encoding::Message<'a, M>>,
) -> usize
where
    M: message::Message<'a>,
{
    encoding::key_len(tag) * values.len()
        + values
            .iter()
            .map(|r| {
                r.map(|v| {
                    let len = v.encoded_len();
                    encoding::encoded_len_varint(len as u64) + len
                })
                .unwrap_or(0)
            })
            .sum::<usize>()
}

#[inline]
pub fn decode<'a, M>(
    _tag: u32,
    wire_type: encoding::WireType,
    _msg_buf: &'a [u8],
    remaining: &mut &'a [u8],
    field: &mut M,
) -> Result<(), error::DecodeError>
where
    M: message::Message<'a>,
{
    encoding::check_wire_type(encoding::WireType::LengthDelimited, wire_type)?;
    *field = decode_single_value(remaining)?;
    Ok(())
}

#[inline]
pub fn decode_optional<'a, M>(
    _tag: u32,
    wire_type: encoding::WireType,
    _msg_buf: &'a [u8],
    remaining: &mut &'a [u8],
    field: &mut Option<M>,
) -> Result<(), error::DecodeError>
where
    M: message::Message<'a>,
{
    encoding::check_wire_type(encoding::WireType::LengthDelimited, wire_type)?;
    *field = Some(decode_single_value(remaining)?);
    Ok(())
}

#[inline]
pub fn decode_single_value<'a, M>(cursor: &mut &'a [u8]) -> Result<M, error::DecodeError>
where
    M: message::Message<'a>,
{
    use bytes::Buf as _;
    let len = encoding::decode_varint(cursor)? as usize;
    if cursor.remaining() >= len {
        let bytes = &cursor[..len];
        let msg = M::decode(bytes)?;
        cursor.advance(len);
        Ok(msg)
    } else {
        Err(error::DecodeError::BufferUnderflow)
    }
}

#[inline]
pub fn decode_repeated<'a, M>(
    tag: u32,
    wire_type: encoding::WireType,
    msg_buf: &'a [u8],
    cursor: &mut &'a [u8],
    field: &mut repeated::Repeated<'a, M, item_encoding::Message<'a, M>>,
) -> Result<(), error::DecodeError>
where
    M: message::Message<'a>,
{
    if field.is_unpopulated() {
        *field = repeated::Repeated::from_msg_buf(tag, msg_buf);
    }
    encoding::skip_field(wire_type, tag, cursor)?;
    Ok(())
}

#[inline]
pub fn clear<'a, M>(_tag: u32, field: &mut M)
where
    M: message::Message<'a>,
{
    field.clear();
}

#[inline]
pub fn clear_optional<'a, M>(_tag: u32, field: &mut Option<M>)
where
    M: message::Message<'a>,
{
    *field = None;
}

#[inline]
pub fn clear_repeated<'a, M>(
    _tag: u32,
    field: &mut repeated::Repeated<'a, M, item_encoding::Message<'a, M>>,
) where
    M: message::Message<'a>,
{
    *field = repeated::Repeated::empty();
}
