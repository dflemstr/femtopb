//! Low-level encoding utility functions and types.
use crate::error;

/// The smallest possible tag value.
pub const MIN_TAG: u32 = 1;
/// The largest possible tag value.
pub const MAX_TAG: u32 = (1 << 29) - 1;

/// All the possible protobuf wire types for encoding fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WireType {
    Varint = 0,
    SixtyFourBit = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    ThirtyTwoBit = 5,
}

impl TryFrom<u64> for WireType {
    type Error = error::DecodeError;

    #[inline]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WireType::Varint),
            1 => Ok(WireType::SixtyFourBit),
            2 => Ok(WireType::LengthDelimited),
            3 => Ok(WireType::StartGroup),
            4 => Ok(WireType::EndGroup),
            5 => Ok(WireType::ThirtyTwoBit),
            _ => Err(error::DecodeError::InvalidWireTypeValue(value)),
        }
    }
}

/// Encodes the provided value as a variable-length encoded integer in LEB128 variable length
/// format.
///
/// The provided `cursor` buffer will be updated to point to just after the encoded integer.
#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encode_varint(mut value: u64, cursor: &mut &mut [u8]) {
    use bytes::BufMut as _;
    loop {
        if value < 0x80 {
            cursor.put_u8(value as u8);
            break;
        } else {
            cursor.put_u8(((value & 0x7F) | 0x80) as u8);
            value >>= 7;
        }
    }
}

/// Decodes a variable-length encoded integer in LEB128 variable length format.
///
/// The provided cursor will, on success, be updated to point just past the decoded integer.
/// On failure, the cursor buffer will not be updated, and still point to the beginning of the
/// variable-length encoded integer.
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
#[inline]
pub fn decode_varint(cursor: &mut &[u8]) -> Result<u64, error::DecodeError> {
    // This function has been unrolled manually to not produce any panic branches no matter which
    // optimization level is used (primary objective), while still performing really well with
    // optimizations enabled (secondary objective).  The performance side to this is probably
    // Premature Optimization®: The Root of All Evil™, but having the function be panic-free is
    // more useful in practice.

    // Some notes as to what has made a difference during testing (looking at disassembly both for
    // this function in isolation and in the inlined context of decoding messages of various
    // sizes and types):
    //
    // * Unrolling the loop for each of the up to 10 bytes prevents a panic check for shift-left
    //   operations (e.g. `byte << (i * 7)` requires checking that `i * 7 < 64`, and panic if not).
    //   This sometimes gets optimized away, but it's nice to have the code be panic-free even
    //   with no optimizations enabled.  The unrolled version lets us use a constant for the
    //   shift-left operation (e.g. `byte << 21`).
    // * Using the `take_first` function has no branches which lead to panics, unlike unchecked
    //   array accesses like `cursor[i]` or the `bytes::Buf` methods.  When optimizations are
    //   enabled, the function gets inlined and the slice indirections get eliminated.
    // * Marking the error branches with `#[cold]` made the generated code generate "jump"
    //   instructions for the error case, and fallthrough code for the happy path.  This likely
    //   makes the BPU on modern CPUs happy, but what do I know...
    // * Handling the last byte (`cursor[9]`) as a special-case where we only want to look at a
    //   single bit makes the compiler less confused about the possible code paths.  Without the
    //   special treatment, the compiler generates code that checks for the "is only one bit set?"
    //   case for every loop.  This might be faster in some cases for what I know, but with the
    //   unrolled version, the generated code seems to be a lot cleaner, with one distinct return
    //   path block for byte 0, one shared block for bytes 1..8, and a distinct one for byte 9.
    //
    // Things I haven't tried yet:
    //
    // * Processing bytes in larger chunks, e.g. u32 or u64, maybe using Duff's Device-style code.
    //   This is maybe a bit hard to accomplish with safe rust, and has alignment problems and
    //   such...

    #[inline]
    #[cold]
    fn buffer_underflow<A>() -> Result<A, error::DecodeError> {
        // Utility function to be able to mark this code path as cold (aka unlikely)
        Err(error::DecodeError::BufferUnderflow)
    }

    #[inline]
    #[cold]
    fn invalid_varint<A>() -> Result<A, error::DecodeError> {
        // Utility function to be able to mark this code path as cold (aka unlikely)
        Err(error::DecodeError::InvalidVarint)
    }

    #[inline]
    fn take_first(slice: &mut &[u8]) -> Result<u8, error::DecodeError> {
        if let Some((byte, rest)) = (*slice).split_first() {
            *slice = rest;
            Ok(*byte)
        } else {
            buffer_underflow()
        }
    }

    let mut value: u64;

    // byte 0
    let byte = take_first(cursor)?;
    value = u64::from(byte & 0b0111_1111);
    if byte & 0b1000_0000 == 0 {
        return Ok(value);
    }

    // byte 1
    let byte = take_first(cursor)?;
    value |= u64::from(byte & 0b0111_1111) << 7;
    if byte & 0b1000_0000 == 0 {
        return Ok(value);
    }

    // byte 2
    let byte = take_first(cursor)?;
    value |= u64::from(byte & 0b0111_1111) << 14;
    if byte & 0b1000_0000 == 0 {
        return Ok(value);
    }

    // byte 3
    let byte = take_first(cursor)?;
    value |= u64::from(byte & 0b0111_1111) << 21;
    if byte & 0b1000_0000 == 0 {
        return Ok(value);
    }

    // byte 4
    let byte = take_first(cursor)?;
    value |= u64::from(byte & 0b0111_1111) << 28;
    if byte & 0b1000_0000 == 0 {
        return Ok(value);
    }

    // byte 5
    let byte = take_first(cursor)?;
    value |= u64::from(byte & 0b0111_1111) << 35;
    if byte & 0b1000_0000 == 0 {
        return Ok(value);
    }

    // byte 6
    let byte = take_first(cursor)?;
    value |= u64::from(byte & 0b0111_1111) << 42;
    if byte & 0b1000_0000 == 0 {
        return Ok(value);
    }

    // byte 7
    let byte = take_first(cursor)?;
    value |= u64::from(byte & 0b0111_1111) << 49;
    if byte & 0b1000_0000 == 0 {
        return Ok(value);
    }

    // byte 8
    let byte = take_first(cursor)?;
    value |= u64::from(byte & 0b0111_1111) << 56;
    if byte & 0b1000_0000 == 0 {
        return Ok(value);
    }

    // byte 9
    let byte = take_first(cursor)?;
    // Here, we only accept a single bit, since we have already seen 63 bits up until this point,
    // and the u64 type obviously only holds 64 bits.
    value |= u64::from(byte & 0b0000_0001) << 63;
    if byte & 0b1111_1110 == 0 {
        return Ok(value);
    }

    // Last byte was too large or had continuation bit set
    invalid_varint()
}

/// Returns the encoded length of the value in LEB128 variable length format.
/// The returned value will be between 1 and 10, inclusive.
#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encoded_len_varint(value: u64) -> usize {
    // Based on [VarintSize64][1].
    // [1]: https://github.com/google/protobuf/blob/3.3.x/src/google/protobuf/io/coded_stream.h#L1301-L1309
    ((((value | 1).leading_zeros() ^ 63) * 9 + 73) / 64) as usize
}

/// Encodes a Protobuf field key, which consists of a wire type designator and
/// the field tag.
#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn encode_key(tag: u32, wire_type: WireType, cursor: &mut &mut [u8]) {
    debug_assert!((MIN_TAG..=MAX_TAG).contains(&tag));
    let key = (tag << 3) | wire_type as u32;
    encode_varint(u64::from(key), cursor);
}

/// Decodes a Protobuf field key, which consists of a wire type designator and
/// the field tag.
#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn decode_key(buf: &mut &[u8]) -> Result<(u32, WireType), error::DecodeError> {
    let key = decode_varint(buf)?;
    if key > u64::from(u32::MAX) {
        return Err(error::DecodeError::InvalidKeyValue(key));
    }
    let wire_type = WireType::try_from(key & 0x7)?;
    let tag = key as u32 >> 3;

    if tag < MIN_TAG {
        return Err(error::DecodeError::InvalidTagValue(tag));
    }

    Ok((tag, wire_type))
}

/// Returns the width of an encoded Protobuf field key with the given tag.
/// The returned width will be between 1 and 5 bytes (inclusive).
#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn key_len(tag: u32) -> usize {
    encoded_len_varint(u64::from(tag << 3))
}

/// Checks that the expected wire type matches the actual wire type,
/// or returns an error result.
#[inline]
#[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub(crate) fn check_wire_type(
    expected: WireType,
    actual: WireType,
) -> Result<(), error::DecodeError> {
    if expected != actual {
        Err(error::DecodeError::UnexpectedWireTypeValue { expected, actual })
    } else {
        Ok(())
    }
}

/// Skips a field of the given wire type and tag.
///
/// On success, the cursor will be updated to point past the skipped field.
/// On failure, the cursor will be in an undefined inconsistent state, since a failure in this
/// function means that the buffer is corrupted.
#[inline]
// This function can not be proven to be panic-free in isolation, likely because of the recursion.
// See https://github.com/dtolnay/no-panic/issues/56
// However, in the context of surrounding code, this panic false negative goes away
// #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
pub fn skip_field(
    wire_type: WireType,
    tag: u32,
    cursor: &mut &[u8],
) -> Result<(), error::DecodeError> {
    let len = match wire_type {
        WireType::Varint => {
            decode_varint(cursor)?;
            0 // decode_varint has already advanced the cursor slice
        }
        WireType::ThirtyTwoBit => 4,
        WireType::SixtyFourBit => 8,
        WireType::LengthDelimited => {
            // decode_varint advances the cursor; now skip more bytes corresponding to
            // the returned value
            decode_varint(cursor)? as usize
        }
        WireType::StartGroup => loop {
            let (inner_tag, inner_wire_type) = decode_key(cursor)?;
            match inner_wire_type {
                WireType::EndGroup => {
                    if inner_tag == tag {
                        break 0;
                    } else {
                        return Err(error::DecodeError::UnexpectedEndGroupTag);
                    }
                }
                _ => skip_field_in_group(inner_wire_type, inner_tag, cursor)?,
            }
        },
        WireType::EndGroup => return Err(error::DecodeError::UnexpectedEndGroupTag),
    };

    if let Some(rest) = (*cursor).get(len..) {
        *cursor = rest;
        Ok(())
    } else {
        Err(error::DecodeError::BufferUnderflow)
    }
}

#[inline(never)]
#[cold]
// This function can not be proven to be panic-free in isolation, likely because of the recursion.
// See https://github.com/dtolnay/no-panic/issues/56
// However, in the context of surrounding code, this panic false negative goes away
// #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
fn skip_field_in_group(
    wire_type: WireType,
    tag: u32,
    cursor: &mut &[u8],
) -> Result<(), error::DecodeError> {
    skip_field(wire_type, tag, cursor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varint() {
        fn check(value: u64, encoded: &[u8]) {
            let mut buf = vec![0; 100];
            let mut buf_slice = buf.as_mut_slice();
            encode_varint(value, &mut buf_slice);
            let remaining = buf_slice.len();
            let encoded_len = buf.len() - remaining;
            assert_eq!(&buf[..encoded_len], encoded);

            assert_eq!(encoded_len_varint(value), encoded.len());

            let mut remaining = encoded;
            let roundtrip_value = decode_varint(&mut remaining).expect("decoding failed");
            assert!(remaining.is_empty());
            assert_eq!(value, roundtrip_value);
        }

        check(2u64.pow(0) - 1, &[0x00]);
        check(2u64.pow(0), &[0x01]);

        check(2u64.pow(7) - 1, &[0x7F]);
        check(2u64.pow(7), &[0x80, 0x01]);
        check(300, &[0xAC, 0x02]);

        check(2u64.pow(14) - 1, &[0xFF, 0x7F]);
        check(2u64.pow(14), &[0x80, 0x80, 0x01]);

        check(2u64.pow(21) - 1, &[0xFF, 0xFF, 0x7F]);
        check(2u64.pow(21), &[0x80, 0x80, 0x80, 0x01]);

        check(2u64.pow(28) - 1, &[0xFF, 0xFF, 0xFF, 0x7F]);
        check(2u64.pow(28), &[0x80, 0x80, 0x80, 0x80, 0x01]);

        check(2u64.pow(35) - 1, &[0xFF, 0xFF, 0xFF, 0xFF, 0x7F]);
        check(2u64.pow(35), &[0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);

        check(2u64.pow(42) - 1, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F]);
        check(2u64.pow(42), &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);

        check(
            2u64.pow(49) - 1,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
        );
        check(
            2u64.pow(49),
            &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
        );

        check(
            2u64.pow(56) - 1,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
        );
        check(
            2u64.pow(56),
            &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
        );

        check(
            2u64.pow(63) - 1,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
        );
        check(
            2u64.pow(63),
            &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
        );

        check(
            u64::MAX,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
        );
    }

    #[test]
    fn varint_overflow() {
        let mut u64_max_plus_one: &[u8] =
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x02];

        decode_varint(&mut u64_max_plus_one).expect_err("decoding u64::MAX + 1 succeeded");
    }
}
