use crate::repeated;
use crate::{encoding, error};
use crate::{item_encoding, message};

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encode<'a, M>(tag: u32, value: &M, cursor: &mut &mut [u8])
where
    M: message::Message<'a>,
{
    encoding::encode_key(tag, encoding::WireType::LengthDelimited, cursor);
    // `usize` is at most 64 bits wide on every target Rust supports, so this widening cast is
    // lossless; a fallible `u64::try_from(..).unwrap()` would only add an unprovable panic branch.
    encoding::encode_varint(value.encoded_len() as u64, cursor);
    value.encode_raw(cursor);
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encode_optional<'a, M>(tag: u32, value: &Option<M>, cursor: &mut &mut [u8])
where
    M: message::Message<'a>,
{
    if let Some(m) = value.as_ref() {
        encode(tag, m, cursor)
    }
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encode_repeated<'a, M>(
    tag: u32,
    values: &repeated::Repeated<'a, M, item_encoding::Message<'a, M>>,
    cursor: &mut &mut [u8],
) where
    M: message::Message<'a>,
{
    for value in values.into_iter().flatten() {
        encode(tag, &value, cursor);
    }
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encoded_len<'a, M>(tag: u32, value: &M) -> usize
where
    M: message::Message<'a>,
{
    let len = value.encoded_len();
    // Lossless widening cast; see the note in `encode`.
    encoding::key_len(tag) + encoding::encoded_len_varint(len as u64) + len
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
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
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encoded_len_repeated<'a, M>(
    tag: u32,
    values: &repeated::Repeated<'a, M, item_encoding::Message<'a, M>>,
) -> usize
where
    M: message::Message<'a>,
{
    values
        .iter()
        .map(|r| {
            r.map(|v| {
                let len = v.encoded_len();
                encoding::key_len(tag) + encoding::encoded_len_varint(len as u64) + len
            })
            .unwrap_or(0)
        })
        .sum::<usize>()
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn decode<'a, M>(
    _tag: u32,
    wire_type: encoding::WireType,
    _msg_buf: &'a [u8],
    _field_start: &'a [u8],
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
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn decode_optional<'a, M>(
    _tag: u32,
    wire_type: encoding::WireType,
    _msg_buf: &'a [u8],
    _field_start: &'a [u8],
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
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn decode_single_value<'a, M>(cursor: &mut &'a [u8]) -> Result<M, error::DecodeError>
where
    M: message::Message<'a>,
{
    let len = encoding::decode_varint(cursor)?;
    let len =
        usize::try_from(len).map_err(|_| error::DecodeError::LengthTooLargeForPlatform(len))?;
    // `split_at_checked` rather than a length test plus the panicking `split_at`: the two are
    // equivalent, but only the former leaves no panic branch for the optimizer to have to prove
    // unreachable.
    let Some((bytes, rest)) = cursor.split_at_checked(len) else {
        return Err(error::DecodeError::BufferUnderflow);
    };
    let msg = M::decode(bytes)?;
    *cursor = rest;
    Ok(msg)
}

#[inline(never)]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn decode_repeated<'a, M>(
    tag: u32,
    wire_type: encoding::WireType,
    msg_buf: &'a [u8],
    field_start: &'a [u8],
    cursor: &mut &'a [u8],
    field: &mut repeated::Repeated<'a, M, item_encoding::Message<'a, M>>,
) -> Result<(), error::DecodeError>
where
    M: message::Message<'a>,
{
    if field.is_unpopulated() {
        *field = repeated::Repeated::from_msg_buf(tag, msg_buf, field_start);
    }
    encoding::skip_field(wire_type, tag, cursor)?;
    field.extend_region(cursor);
    Ok(())
}

#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
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
