use crate::encoding;

/// A Protobuf message decoding error.
///
/// `DecodeError` indicates that the input buffer does not contain a valid
/// Protobuf message. The error details should be considered 'best effort': in
/// general it is not possible to exactly pinpoint why data is malformed.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DecodeError {
    InvalidVarint,
    VarintTooLarge(u64),
    RecursionLimitReached,
    BufferUnderflow,
    DelimitedLengthExceeded,
    UnexpectedEndGroupTag,
    InvalidWireTypeValue(u64),
    UnexpectedWireTypeValue {
        actual: encoding::WireType,
        expected: encoding::WireType,
    },
    UnexpectedTagValue(u32),
    InvalidKeyValue(u64),
    InvalidTagValue(u32),
    InvalidUtf8(core::str::Utf8Error),
}

/// A Protobuf message encoding error.
///
/// `EncodeError` always indicates that a message failed to encode because the
/// provided buffer had insufficient capacity. Message encoding is otherwise
/// infallible.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EncodeError {
    required: usize,
    remaining: usize,
}

impl EncodeError {
    pub fn new(required: usize, remaining: usize) -> Self {
        Self {
            required,
            remaining,
        }
    }
}
