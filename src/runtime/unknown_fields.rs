use crate::{encoding, error, unknown_fields};
use core::mem;

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

/// Walks the retained message buffer and invokes `visit` with the raw bytes of each field whose tag
/// is *not* known to the surrounding message — i.e. each unknown field that must be preserved.
///
/// Iteration stops at the first malformed field. The buffer was already parsed successfully during
/// decoding, so for data produced by this crate that never happens; the early-out just keeps the
/// function total and panic-free.
#[inline]
fn for_each_unknown_field<F>(field: &unknown_fields::UnknownFields, mut visit: F)
where
    F: FnMut(&[u8]),
{
    let known_tags = field.known_tags();
    let mut buf = field.msg_buf();
    while !buf.is_empty() {
        let before = buf;
        let Ok((tag, wire_type)) = encoding::decode_key(&mut buf) else {
            break;
        };
        if encoding::skip_field(wire_type, tag, &mut buf).is_err() {
            break;
        }
        let field_len = before.len() - buf.len();
        if !known_tags.contains(&tag) {
            if let Some((field_bytes, _)) = before.split_at_checked(field_len) {
                visit(field_bytes);
            }
        }
    }
}

/// The number of bytes needed to re-encode the preserved unknown fields.
#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encoded_len(field: &unknown_fields::UnknownFields) -> usize {
    let mut total = 0;
    for_each_unknown_field(field, |bytes| total += bytes.len());
    total
}

/// Copies the preserved unknown fields verbatim into the output buffer, so that decoding and then
/// re-encoding a message round-trips fields that were not known to the current schema.
#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encode(field: &unknown_fields::UnknownFields, cursor: &mut &mut [u8]) {
    for_each_unknown_field(field, |bytes| {
        let out = mem::take(cursor);
        // `encoded_len` reserves exactly these bytes, so the split always succeeds; if a caller
        // under-allocated we simply stop writing rather than panic (`cursor` is already empty).
        if let Some((dst, rest)) = out.split_at_mut_checked(bytes.len()) {
            dst.copy_from_slice(bytes);
            *cursor = rest;
        }
    });
}

/// Clears the unknown fields, so a cleared message re-encodes without them.
#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn clear(field: &mut unknown_fields::UnknownFields) {
    *field = unknown_fields::UnknownFields::empty();
}
