//! Definition of the `Oneof` trait and related types.
use crate::{encoding, error};

/// A type that encapsulates one of many mutually exclusive fields.
pub trait Oneof<'a>: Sized {
    /// Encodes the currently active field of this Oneof to the specified buffer.
    ///
    /// The cursor buffer will be updated to point past the value that was written to the buffer.
    fn encode(&self, cursor: &mut &mut [u8]);

    /// How much space will be required to encode the currently active field.
    fn encoded_len(&self) -> usize;

    /// Decodes a field that is part of this oneof, and updates the internal state so that the
    /// new field becomes active.
    ///
    /// The cursor buffer will be updated to point past the value that was read from the buffer.
    fn decode(
        tag: u32,
        wire_type: encoding::WireType,
        msg_buf: &'a [u8],
        cursor: &mut &'a [u8],
    ) -> Result<Self, error::DecodeError>;
}
