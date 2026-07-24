//! Common error type definitions
use crate::encoding;

/// A Protobuf message decoding error.
///
/// `DecodeError` indicates that the input buffer does not contain a valid
/// Protobuf message. The error details should be considered 'best effort': in
/// general it is not possible to exactly pinpoint why data is malformed.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "thiserror", derive(thiserror_no_std::Error))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum DecodeError {
    /// We were unable to decode a varint, because the last byte overflowed a 64-bit integer; the
    /// buffer is likely corrupt.
    #[cfg_attr(feature = "thiserror", error("Unable to decode varint: the last byte overflowed a 64-bit integer. The buffer is likely corrupt"))]
    InvalidVarint,
    /// The decoded varint was larger than expected; the too-large value is enclosed.
    #[cfg_attr(feature = "thiserror", error("Varint larger than expected: {0}"))]
    VarintTooLarge(u64),
    /// A length-delimited field declared a length that is a valid `u64` but does not fit in a
    /// `usize` on the current platform, so the field cannot be addressed on this architecture.
    ///
    /// Unlike [`VarintTooLarge`](Self::VarintTooLarge), this does not necessarily indicate a
    /// corrupt buffer: the same message may decode successfully on a target with a wider `usize`
    /// (for example a 64-bit platform). The offending length is enclosed.
    #[cfg_attr(feature = "thiserror", error("Length {0} does not fit in a usize on this platform"))]
    LengthTooLargeForPlatform(u64),
    /// The provided buffer was too short to be able to decode the desired data.
    #[cfg_attr(feature = "thiserror", error("Provided buffer is too short"))]
    BufferUnderflow,
    /// An end group tag was encountered without a matching start group tag.
    #[cfg_attr(feature = "thiserror", error("End group tag encountered without matching start group tag"))]
    UnexpectedEndGroupTag,
    /// The specified wire type value was too large; the too-large value is enclosed.
    #[cfg_attr(feature = "thiserror", error("Wire type value too large: {0}"))]
    InvalidWireTypeValue(u64),
    /// We expected a different wire type value than the one that was encountered.
    #[cfg_attr(feature = "thiserror",
        error("Unexpected wire type: {}. Expected: {}", *actual as u8, *expected as u8))]
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
    #[cfg_attr(feature = "thiserror", error("Unexpected tag value: {0}"))]
    UnexpectedTagValue(u32),
    /// The encountered key value was out of range; the out-of-range value is enclosed.
    #[cfg_attr(feature = "thiserror", error("Key value out of range: {0}"))]
    InvalidKeyValue(u64),
    /// The encountered tag value was out of range; the out-of-range value is enclosed.
    ///
    /// This is different from `InvalidKeyValue` in the sense that the key had a valid wire type,
    /// but invalid field tag value.
    #[cfg_attr(feature = "thiserror", error("Key value out of range: {0}"))]
    InvalidTagValue(u32),
    /// The encountered string field does not contain valid UTF-8 data.
    #[cfg_attr(feature = "thiserror",
        error("Invalid UTF-8 data: Valid up to {}. Error length: {}",
            valid_up_to,
            OptionalLen(error_len)))]
    InvalidUtf8 {
        valid_up_to: usize,
        error_len: Option<usize>,
    },
}

/// Formats an optional error length for the [`DecodeError::InvalidUtf8`] message, rendering
/// `Some(n)` as `n` and `None` as `N/A` without any allocation.
#[cfg(feature = "thiserror")]
struct OptionalLen<'a>(&'a Option<usize>);

#[cfg(feature = "thiserror")]
impl core::fmt::Display for OptionalLen<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            Some(len) => write!(f, "{len}"),
            None => f.write_str("N/A"),
        }
    }
}

/// A Protobuf message encoding error.
///
/// `EncodeError` always indicates that a message failed to encode because the
/// provided buffer had insufficient capacity. Message encoding is otherwise
/// infallible.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "thiserror", derive(thiserror_no_std::Error))]
#[cfg_attr(feature = "thiserror", error("encode error: required {required} bytes but only {remaining} remaining"))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

#[cfg(all(test, feature = "thiserror"))]
mod tests {
    use super::*;

    #[test]
    fn invalid_utf8_display_with_error_len() {
        let e = DecodeError::InvalidUtf8 {
            valid_up_to: 3,
            error_len: Some(2),
        };
        assert_eq!(
            e.to_string(),
            "Invalid UTF-8 data: Valid up to 3. Error length: 2"
        );
    }

    #[test]
    fn invalid_utf8_display_without_error_len() {
        let e = DecodeError::InvalidUtf8 {
            valid_up_to: 3,
            error_len: None,
        };
        assert_eq!(
            e.to_string(),
            "Invalid UTF-8 data: Valid up to 3. Error length: N/A"
        );
    }

    #[test]
    fn length_too_large_for_platform_display_is_distinct() {
        // The platform-length error must not be confused with the generic varint-too-large error.
        assert_ne!(
            DecodeError::VarintTooLarge(42).to_string(),
            DecodeError::LengthTooLargeForPlatform(42).to_string()
        );
        assert_eq!(
            DecodeError::LengthTooLargeForPlatform(42).to_string(),
            "Length 42 does not fit in a usize on this platform"
        );
    }
}
