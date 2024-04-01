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
#[inline]
pub fn decode_varint(cursor: &mut &[u8]) -> Result<u64, error::DecodeError> {
    use bytes::Buf as _;

    let mut value: u64 = 0;
    for (idx, byte) in cursor.into_iter().copied().take(10).enumerate() {
        value |= u64::from(byte & 0x7F) << (idx * 7);
        if byte <= 0x7F {
            // Check for u64::MAX overflow. See [`ConsumeVarint`][1] for details.
            // [1]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
            return if idx == 9 && byte >= 0x02 {
                Err(error::DecodeError::InvalidVarint)
            } else {
                cursor.advance(idx + 1);
                Ok(value)
            };
        }
    }

    // We have overrun the maximum size of a varint (10 bytes) or the final byte caused an overflow.
    // Assume the data is corrupt.
    Err(error::DecodeError::InvalidVarint)
}

/// Returns the encoded length of the value in LEB128 variable length format.
/// The returned value will be between 1 and 10, inclusive.
#[inline]
pub fn encoded_len_varint(value: u64) -> usize {
    // Based on [VarintSize64][1].
    // [1]: https://github.com/google/protobuf/blob/3.3.x/src/google/protobuf/io/coded_stream.h#L1301-L1309
    ((((value | 1).leading_zeros() ^ 63) * 9 + 73) / 64) as usize
}

/// Encodes a Protobuf field key, which consists of a wire type designator and
/// the field tag.
#[inline]
pub fn encode_key(tag: u32, wire_type: WireType, cursor: &mut &mut [u8]) {
    debug_assert!((MIN_TAG..=MAX_TAG).contains(&tag));
    let key = (tag << 3) | wire_type as u32;
    encode_varint(u64::from(key), cursor);
}

/// Decodes a Protobuf field key, which consists of a wire type designator and
/// the field tag.
#[inline]
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
pub fn key_len(tag: u32) -> usize {
    encoded_len_varint(u64::from(tag << 3))
}

/// Checks that the expected wire type matches the actual wire type,
/// or returns an error result.
#[inline]
pub fn check_wire_type(expected: WireType, actual: WireType) -> Result<(), error::DecodeError> {
    if expected != actual {
        return Err(error::DecodeError::UnexpectedWireTypeValue { expected, actual });
    }
    Ok(())
}

/// Skips a field of the given wire type and tag.
///
/// On success, the cursor will be updated to point past the skipped field.
/// On failure, the cursor will be in an undefined inconsistent state, since a failure in this
/// function means that the buffer is corrupted.
#[inline]
pub fn skip_field(
    wire_type: WireType,
    tag: u32,
    cursor: &mut &[u8],
) -> Result<(), error::DecodeError> {
    use bytes::Buf as _;

    let len = match wire_type {
        WireType::Varint => decode_varint(cursor).map(|_| 0)?,
        WireType::ThirtyTwoBit => 4,
        WireType::SixtyFourBit => 8,
        WireType::LengthDelimited => decode_varint(cursor)?,
        WireType::StartGroup => loop {
            let (inner_tag, inner_wire_type) = decode_key(cursor)?;
            match inner_wire_type {
                WireType::EndGroup => {
                    if inner_tag != tag {
                        return Err(error::DecodeError::UnexpectedEndGroupTag);
                    }
                    break 0;
                }
                _ => skip_field(inner_wire_type, inner_tag, cursor)?,
            }
        },
        WireType::EndGroup => return Err(error::DecodeError::UnexpectedEndGroupTag),
    };

    if len > cursor.remaining() as u64 {
        return Err(error::DecodeError::BufferUnderflow);
    }

    cursor.advance(len as usize);
    Ok(())
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
