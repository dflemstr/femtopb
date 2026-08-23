//! The `Deferred` wrapper type and related types.
use crate::{error, message};
use core::marker;
use core::mem;

/// A `Deferred<A>` allows deferred decoding/encoding of a message of type `A`.
///
/// Use `Deferred::decode()` to do a deferred decode of an `A`.
///
/// The wrapper holds the sub-message's bytes and never looks inside them unless asked to. Encoding
/// one writes those bytes back out verbatim: a deferred field round-trips exactly as it arrived,
/// and re-encoding costs a copy rather than a decode plus a re-encode.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Deferred<'a, A>
where
    A: message::Message<'a>,
{
    msg_buf: &'a [u8],
    phantom: marker::PhantomData<A>,
}

impl<'a, A> Deferred<'a, A>
where
    A: message::Message<'a>,
{
    /// Performs deferred decoding of an `A`, returning the parsed message on success.
    pub fn decode(&self) -> Result<A, error::DecodeError> {
        A::decode(self.msg_buf)
    }

    /// The raw, still-undecoded bytes of the sub-message.
    #[must_use]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.msg_buf
    }
}

impl<'a, A> PartialEq for Deferred<'a, A>
where
    A: message::Message<'a> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        // TODO: is this a good idea? This implementation might be expensive for large messages
        self.decode().eq(&other.decode())
    }
}

impl<'a, A> message::Message<'a> for Deferred<'a, A>
where
    A: message::Message<'a>,
{
    fn encode_raw(&self, cursor: &mut &mut [u8]) {
        let out = mem::take(cursor);
        // `encoded_len` reserves exactly these bytes, so the split always succeeds; if a caller
        // under-allocated we simply stop writing rather than panic (`cursor` is already empty).
        if let Some((dst, rest)) = out.split_at_mut_checked(self.msg_buf.len()) {
            dst.copy_from_slice(self.msg_buf);
            *cursor = rest;
        }
    }

    fn encoded_len(&self) -> usize {
        self.msg_buf.len()
    }

    fn decode(msg_buf: &'a [u8]) -> Result<Self, error::DecodeError>
    where
        Self: Sized,
    {
        let phantom = marker::PhantomData;
        Ok(Self { msg_buf, phantom })
    }

    fn clear(&mut self) {
        self.msg_buf = &[];
    }
}
