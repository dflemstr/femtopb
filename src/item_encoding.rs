//! Item encodings for lazily-parsed types like `Repeated` and `Packed`.
//!
//! Most of the items in this module should be fairly self-explanatory.
use crate::enumeration;
use crate::message;
use crate::runtime;
use crate::{encoding, error};
use core::marker;

/// An item encoding for an embedded type.
pub trait ItemEncoding<'a, A>
where
    A: 'a,
{
    const WIRE_TYPE: encoding::WireType;

    fn decode_single_value(cursor: &mut &'a [u8]) -> Result<A, error::DecodeError>;
}

enum Empty {}

pub struct Bool {
    _empty: Empty,
}

pub struct Int32 {
    _empty: Empty,
}

pub struct Int64 {
    _empty: Empty,
}

pub struct SInt32 {
    _empty: Empty,
}

pub struct SInt64 {
    _empty: Empty,
}

pub struct UInt32 {
    _empty: Empty,
}

pub struct UInt64 {
    _empty: Empty,
}

pub struct Double {
    _empty: Empty,
}

pub struct Float {
    _empty: Empty,
}

pub struct Fixed32 {
    _empty: Empty,
}

pub struct Fixed64 {
    _empty: Empty,
}

pub struct SFixed32 {
    _empty: Empty,
}

pub struct SFixed64 {
    _empty: Empty,
}

pub struct Bytes {
    _empty: Empty,
}

pub struct String {
    _empty: Empty,
}

pub struct Enum<A>
where
    A: enumeration::Enumeration,
{
    _empty: Empty,
    _phantom: marker::PhantomData<A>,
}

pub struct Message<'a, A>
where
    A: message::Message<'a>,
{
    _empty: Empty,
    _phantom: marker::PhantomData<&'a A>,
}

/// Implement `ItemEncoding<'a, $ty>` for `$marker` using the runtime module at `$path`.
macro_rules! runtime_scalar_impl {
    ($scalar:ident, $ty:ty, $marker:ty) => {
        impl ItemEncoding<'_, $ty> for $marker {
            const WIRE_TYPE: encoding::WireType = runtime::scalar::$scalar::WIRE_TYPE;

            fn decode_single_value(cursor: &mut &[u8]) -> Result<$ty, error::DecodeError> {
                runtime::scalar::$scalar::decode_single_value(cursor)
            }
        }
    };
    ($lt:lifetime, $scalar:ident, $ty:ty, $marker:ty) => {
        impl<$lt> ItemEncoding<$lt, $ty> for $marker {
            const WIRE_TYPE: encoding::WireType = runtime::scalar::$scalar::WIRE_TYPE;

            fn decode_single_value(cursor: &mut &$lt[u8]) -> Result<$ty, error::DecodeError> {
                runtime::scalar::$scalar::decode_single_value(cursor)
            }
        }
    };
}

runtime_scalar_impl!(bool, bool, Bool);
runtime_scalar_impl!(int32, i32, Int32);
runtime_scalar_impl!(int64, i64, Int64);
runtime_scalar_impl!(sint32, i32, SInt32);
runtime_scalar_impl!(sint64, i64, SInt64);
runtime_scalar_impl!(uint32, u32, UInt32);
runtime_scalar_impl!(uint64, u64, UInt64);
runtime_scalar_impl!(double, f64, Double);
runtime_scalar_impl!(float, f32, Float);
runtime_scalar_impl!(fixed32, u32, Fixed32);
runtime_scalar_impl!(fixed64, u64, Fixed64);
runtime_scalar_impl!(sfixed32, i32, SFixed32);
runtime_scalar_impl!(sfixed64, i64, SFixed64);

runtime_scalar_impl!('a, bytes, &'a [u8], Bytes);
runtime_scalar_impl!('a, string, &'a str, String);

impl<'a, A> ItemEncoding<'a, enumeration::EnumValue<A>> for Enum<A>
where
    A: enumeration::Enumeration + 'a,
{
    const WIRE_TYPE: encoding::WireType = encoding::WireType::Varint;

    fn decode_single_value(
        cursor: &mut &'a [u8],
    ) -> Result<enumeration::EnumValue<A>, error::DecodeError> {
        runtime::enumeration::decode_single_value(cursor)
    }
}

impl<'a, A> ItemEncoding<'a, A> for Message<'a, A>
where
    A: message::Message<'a>,
{
    const WIRE_TYPE: encoding::WireType = encoding::WireType::LengthDelimited;

    fn decode_single_value(cursor: &mut &'a [u8]) -> Result<A, error::DecodeError> {
        runtime::message::decode_single_value(cursor)
    }
}
