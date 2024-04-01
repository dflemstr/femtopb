//! The `Deferred` wrapper type and related types.
use crate::{error, message};
use core::marker;

/// A `Deferred<A>` allows deferred decoding/encoding of a message of type `A`.
///
/// Use `Deferred::decode()` to do a deferred decode of an `A`.
#[derive(Clone, Debug)]
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
        if let Ok(msg) = self.decode() {
            msg.encode_raw(cursor);
        }
    }

    fn encoded_len(&self) -> usize {
        if let Ok(msg) = self.decode() {
            msg.encoded_len()
        } else {
            0
        }
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
