//! Common error type definitions
use crate::encoding;

/// A Protobuf message decoding error.
///
/// `DecodeError` indicates that the input buffer does not contain a valid
/// Protobuf message. The error details should be considered 'best effort': in
/// general it is not possible to exactly pinpoint why data is malformed.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DecodeError {
    /// We were unable to decode a varint, because the last byte overflowed a 64-bit integer; the
    /// buffer is likely corrupt.
    InvalidVarint,
    /// The decoded varint was larger than expected; the too-large value is enclosed.
    VarintTooLarge(u64),
    /// The provided buffer was too short to be able to decode the desired data.
    BufferUnderflow,
    /// An end group tag was encountered without a matching start group tag.
    UnexpectedEndGroupTag,
    /// The specified wire type value was too large; the too-large value is enclosed.
    InvalidWireTypeValue(u64),
    /// We expected a different wire type value than the one that was encountered.
    UnexpectedWireTypeValue {
        /// The encountered wire type from the decoded buffer.
        actual: encoding::WireType,
        /// The wire type we were expecting to see.
        ///
        /// Other wire types might also be expected for backwards compatibility reasons, but
        /// this indicates the main "happy path" wire type that was expected.
        expected: encoding::WireType,
    },
    /// The encountered tag was different from the one we were expecting; the wrong tag value is
    /// enclosed.
    UnexpectedTagValue(u32),
    /// The encountered key value was out of range; the out-of-range value is enclosed.
    InvalidKeyValue(u64),
    /// The encountered tag value was out of range; the out-of-range value is enclosed.
    ///
    /// This is different from `InvalidKeyValue` in the sense that the key had a valid wire type,
    /// but invalid field tag value.
    InvalidTagValue(u32),
    /// The encountered string field does not contain valid UTF-8 data.
    InvalidUtf8(core::str::Utf8Error),
}

/// A Protobuf message encoding error.
///
/// `EncodeError` always indicates that a message failed to encode because the
/// provided buffer had insufficient capacity. Message encoding is otherwise
/// infallible.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EncodeError {
    /// How much space was required for the encode operation to complete
    pub required: usize,
    /// How much space actually remains in the buffer
    pub remaining: usize,
}

impl EncodeError {
    /// Creates a new `EncodeError` with fields as documented in the struct definition.
    pub fn new(required: usize, remaining: usize) -> Self {
        Self {
            required,
            remaining,
        }
    }
}
