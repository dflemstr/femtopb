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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding::WireType;

    /// The wire type each element encoding advertises. `Repeated`/`Packed` route on this constant to
    /// decide the scalar-vs-packed branch, so pinning it guards against a silent mis-tagging.
    #[test]
    fn wire_types_match_the_protobuf_spec() {
        assert_eq!(Bool::WIRE_TYPE, WireType::Varint);
        assert_eq!(Int32::WIRE_TYPE, WireType::Varint);
        assert_eq!(Int64::WIRE_TYPE, WireType::Varint);
        assert_eq!(SInt32::WIRE_TYPE, WireType::Varint);
        assert_eq!(SInt64::WIRE_TYPE, WireType::Varint);
        assert_eq!(UInt32::WIRE_TYPE, WireType::Varint);
        assert_eq!(UInt64::WIRE_TYPE, WireType::Varint);

        assert_eq!(Double::WIRE_TYPE, WireType::SixtyFourBit);
        assert_eq!(Fixed64::WIRE_TYPE, WireType::SixtyFourBit);
        assert_eq!(SFixed64::WIRE_TYPE, WireType::SixtyFourBit);

        assert_eq!(Float::WIRE_TYPE, WireType::ThirtyTwoBit);
        assert_eq!(Fixed32::WIRE_TYPE, WireType::ThirtyTwoBit);
        assert_eq!(SFixed32::WIRE_TYPE, WireType::ThirtyTwoBit);

        assert_eq!(Bytes::WIRE_TYPE, WireType::LengthDelimited);
        assert_eq!(String::WIRE_TYPE, WireType::LengthDelimited);
        assert_eq!(
            <Message<'_, Msg> as ItemEncoding<'_, Msg>>::WIRE_TYPE,
            WireType::LengthDelimited
        );

        // Enumerations travel as varints, just like the integer scalars.
        assert_eq!(
            <Enum<Color> as ItemEncoding<'_, enumeration::EnumValue<Color>>>::WIRE_TYPE,
            WireType::Varint
        );
    }

    #[test]
    fn scalar_decode_single_value_advances_the_cursor() {
        // int32 150 is the two varint bytes 0x96 0x01; decoding must consume exactly those.
        let mut cursor: &[u8] = &[0x96, 0x01, 0xFF];
        assert_eq!(Int32::decode_single_value(&mut cursor).unwrap(), 150);
        assert_eq!(cursor, &[0xFF]);
    }

    #[test]
    fn string_decode_single_value_reads_a_length_delimited_chunk() {
        let mut cursor: &[u8] = &[0x02, b'h', b'i', 0x00];
        assert_eq!(String::decode_single_value(&mut cursor).unwrap(), "hi");
        assert_eq!(cursor, &[0x00]);
    }

    #[test]
    fn enum_decode_single_value_maps_known_and_unknown() {
        let mut known: &[u8] = &[0x01];
        assert_eq!(
            Enum::<Color>::decode_single_value(&mut known).unwrap(),
            enumeration::EnumValue::Known(Color::Green)
        );
        let mut unknown: &[u8] = &[0x09];
        assert_eq!(
            Enum::<Color>::decode_single_value(&mut unknown).unwrap(),
            enumeration::EnumValue::Unknown(9)
        );
    }

    // A minimal `Enumeration` hand-implemented so these tests don't depend on the derive macro.
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    enum Color {
        #[default]
        Red,
        Green,
        Blue,
    }

    impl enumeration::Enumeration for Color {
        fn encode(&self) -> i32 {
            *self as i32
        }

        fn decode(value: i32) -> enumeration::EnumValue<Self> {
            match value {
                0 => enumeration::EnumValue::Known(Color::Red),
                1 => enumeration::EnumValue::Known(Color::Green),
                2 => enumeration::EnumValue::Known(Color::Blue),
                other => enumeration::EnumValue::Unknown(other),
            }
        }
    }

    // A trivial `Message` so the composite `Message` item encoding has a concrete type to name.
    #[derive(Clone, Debug, Default, PartialEq)]
    struct Msg;

    impl<'a> message::Message<'a> for Msg {
        fn encoded_len(&self) -> usize {
            0
        }

        fn encode_raw(&self, _cursor: &mut &mut [u8]) {}

        fn decode(_buf: &'a [u8]) -> Result<Self, error::DecodeError> {
            Ok(Msg)
        }

        fn clear(&mut self) {}
    }
}
