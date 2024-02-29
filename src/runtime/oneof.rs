use crate::{encoding, error, oneof};

#[inline]
pub fn encode<'a, O>(value: &Option<O>, cursor: &mut &mut [u8])
where
    O: oneof::Oneof<'a>,
{
    if let Some(v) = value {
        v.encode(cursor);
    }
}

#[inline]
pub fn encoded_len<'a, O>(value: &Option<O>) -> usize
where
    O: oneof::Oneof<'a>,
{
    if let Some(v) = value {
        v.encoded_len()
    } else {
        0
    }
}

#[inline]
pub fn decode<'a, O>(
    tag: u32,
    wire_type: encoding::WireType,
    msg_buf: &'a [u8],
    cursor: &mut &'a [u8],
    field: &mut Option<O>,
) -> Result<(), error::DecodeError>
where
    O: oneof::Oneof<'a>,
{
    *field = Some(O::decode(tag, wire_type, msg_buf, cursor)?);
    Ok(())
}

#[inline]
pub fn clear<'a, O>(value: &mut Option<O>)
where
    O: oneof::Oneof<'a>,
{
    *value = None;
}
