//! Definition of the core `Message` trait.
use crate::encoding;
use crate::error;

/// A message type that can be encoded and decoded to a buffer using the protobuf wire format.
pub trait Message<'a>: Clone {
    /// Encodes the message to a buffer pointed-to by the specified cursor.
    ///
    /// This method will panic if the buffer pointed to by `cursor` has insufficient capacity.
    ///
    /// Meant to be used only by `Message` implementations.
    fn encode_raw(&self, cursor: &mut &mut [u8]);

    /// Returns the encoded length of the message without a length delimiter.
    fn encoded_len(&self) -> usize;

    /// Encodes the message to a buffer.
    ///
    /// If successful, the passed in mutable buffer reference will be updated to point just after
    /// the end of the written message; it will point to the empty slice if the buffer space was
    /// used up completely.  You can use this if you want to chain multiple writes to the same
    /// buffer.  Consider, however, that messages are not self-delimiting by default.  Consider
    /// using `encode_length_delimited` for cases where delimited messages are desired.
    ///
    /// An error will be returned if the buffer does not have sufficient capacity.
    fn encode(&self, cursor: &mut &mut [u8]) -> Result<(), error::EncodeError> {

        let required = self.encoded_len();
        let remaining = cursor.len();
        if required > remaining {
            return Err(error::EncodeError::new(required, remaining));
        }

        self.encode_raw(cursor);
        Ok(())
    }

    /// Encodes the message with a length-delimiter to a buffer.
    ///
    /// If successful, the passed in mutable buffer reference will be updated to point just after
    /// the end of the written message; it will point to the empty slice if the buffer space was
    /// used up completely.  You can use this if you want to chain multiple writes to the same
    /// buffer.
    ///
    /// An error will be returned if the buffer does not have sufficient capacity.
    fn encode_length_delimited(&self, cursor: &mut &mut [u8]) -> Result<(), error::EncodeError> {
        let len = self.encoded_len();
        let required = len + encoding::encoded_len_varint(len as u64);
        let remaining = cursor.len();
        if required > remaining {
            return Err(error::EncodeError::new(required, remaining));
        }
        encoding::encode_varint(len as u64, cursor);
        self.encode_raw(cursor);
        Ok(())
    }

    /// Decodes an instance of the message from a buffer.
    ///
    /// The entire buffer will be consumed.
    // TODO: consider turning this function into a method instead, with "merge" semantics...
    // however, it won't be straight-forward to handle the lifetimes then
    fn decode(buf: &'a [u8]) -> Result<Self, error::DecodeError>
    where
        Self: Sized;

    /// Decodes a length-delimited instance of the message from the buffer.
    ///
    /// If successful, the passed in mutable buffer reference will be updated to point just after
    /// the end of the decoded message; it will point to the empty slice if the buffer space was
    /// used up completely.  You can use this if you want to chain multiple reads from the same
    /// buffer.
    fn decode_length_delimited<B>(buf: &mut &'a [u8]) -> Result<Self, error::DecodeError>
    where
        Self: Sized,
    {
        let len = encoding::decode_varint(buf)?;
        let len = usize::try_from(len).map_err(|_| error::DecodeError::VarintTooLarge(len))?;
        let (start, rest) = buf.split_at(len);
        let message = Self::decode(&start)?;
        *buf = rest;
        Ok(message)
    }

    /// Clears the message, resetting all fields to their default.
    fn clear(&mut self);
}
