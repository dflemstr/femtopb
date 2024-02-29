use crate::{encoding, error};

pub trait Oneof<'a>: Sized {
    fn encode(&self, cursor: &mut &mut [u8]);

    fn encoded_len(&self) -> usize;

    fn decode(
        tag: u32,
        wire_type: encoding::WireType,
        msg_buf: &'a [u8],
        cursor: &mut &'a [u8],
    ) -> Result<Self, error::DecodeError>;
}
