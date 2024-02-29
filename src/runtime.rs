mod macros;

pub mod enumeration;
pub mod message;
pub mod oneof;
pub mod scalar;
pub mod unknown_fields;

//
// #[cfg(test)]
// mod test {
//     use alloc::string::ToString;
//     use core::borrow::Borrow;
//     use core::fmt::Debug;
//     use core::u64;
//
//     use ::bytes::{Bytes, BytesMut};
//     use proptest::{prelude::*, test_runner::TestCaseResult};
//
//     use crate::encoding::*;
//
//     pub fn check_type<T, B>(
//         value: T,
//         tag: u32,
//         wire_type: WireType,
//         encode: fn(u32, &B, &mut BytesMut),
//         merge: fn(WireType, &mut T, &mut Bytes, DecodeContext) -> Result<(), DecodeError>,
//         encoded_len: fn(u32, &B) -> usize,
//     ) -> TestCaseResult
//         where
//             T: Debug + Default + PartialEq + Borrow<B>,
//             B: ?Sized,
//     {
//         prop_assume!((MIN_TAG..=MAX_TAG).contains(&tag));
//
//         let expected_len = encoded_len(tag, value.borrow());
//
//         let mut buf = BytesMut::with_capacity(expected_len);
//         encode(tag, value.borrow(), &mut buf);
//
//         let mut buf = buf.freeze();
//
//         prop_assert_eq!(
//             buf.remaining(),
//             expected_len,
//             "encoded_len wrong; expected: {}, actual: {}",
//             expected_len,
//             buf.remaining()
//         );
//
//         if !buf.has_remaining() {
//             // Short circuit for empty packed values.
//             return Ok(());
//         }
//
//         let (decoded_tag, decoded_wire_type) =
//             decode_key(&mut buf).map_err(|error| TestCaseError::fail(error.to_string()))?;
//         prop_assert_eq!(
//             tag,
//             decoded_tag,
//             "decoded tag does not match; expected: {}, actual: {}",
//             tag,
//             decoded_tag
//         );
//
//         prop_assert_eq!(
//             wire_type,
//             decoded_wire_type,
//             "decoded wire type does not match; expected: {:?}, actual: {:?}",
//             wire_type,
//             decoded_wire_type,
//         );
//
//         match wire_type {
//             WireType::SixtyFourBit if buf.remaining() != 8 => Err(TestCaseError::fail(format!(
//                 "64bit wire type illegal remaining: {}, tag: {}",
//                 buf.remaining(),
//                 tag
//             ))),
//             WireType::ThirtyTwoBit if buf.remaining() != 4 => Err(TestCaseError::fail(format!(
//                 "32bit wire type illegal remaining: {}, tag: {}",
//                 buf.remaining(),
//                 tag
//             ))),
//             _ => Ok(()),
//         }?;
//
//         let mut roundtrip_value = T::default();
//         merge(
//             wire_type,
//             &mut roundtrip_value,
//             &mut buf,
//             DecodeContext::default(),
//         )
//             .map_err(|error| TestCaseError::fail(error.to_string()))?;
//
//         prop_assert!(
//             !buf.has_remaining(),
//             "expected buffer to be empty, remaining: {}",
//             buf.remaining()
//         );
//
//         prop_assert_eq!(value, roundtrip_value);
//
//         Ok(())
//     }
//
//     pub fn check_collection_type<T, B, E, M, L>(
//         value: T,
//         tag: u32,
//         wire_type: WireType,
//         encode: E,
//         mut merge: M,
//         encoded_len: L,
//     ) -> TestCaseResult
//         where
//             T: Debug + Default + PartialEq + Borrow<B>,
//             B: ?Sized,
//             E: FnOnce(u32, &B, &mut BytesMut),
//             M: FnMut(WireType, &mut T, &mut Bytes, DecodeContext) -> Result<(), DecodeError>,
//             L: FnOnce(u32, &B) -> usize,
//     {
//         prop_assume!((MIN_TAG..=MAX_TAG).contains(&tag));
//
//         let expected_len = encoded_len(tag, value.borrow());
//
//         let mut buf = BytesMut::with_capacity(expected_len);
//         encode(tag, value.borrow(), &mut buf);
//
//         let mut buf = buf.freeze();
//
//         prop_assert_eq!(
//             buf.remaining(),
//             expected_len,
//             "encoded_len wrong; expected: {}, actual: {}",
//             expected_len,
//             buf.remaining()
//         );
//
//         let mut roundtrip_value = Default::default();
//         while buf.has_remaining() {
//             let (decoded_tag, decoded_wire_type) =
//                 decode_key(&mut buf).map_err(|error| TestCaseError::fail(error.to_string()))?;
//
//             prop_assert_eq!(
//                 tag,
//                 decoded_tag,
//                 "decoded tag does not match; expected: {}, actual: {}",
//                 tag,
//                 decoded_tag
//             );
//
//             prop_assert_eq!(
//                 wire_type,
//                 decoded_wire_type,
//                 "decoded wire type does not match; expected: {:?}, actual: {:?}",
//                 wire_type,
//                 decoded_wire_type
//             );
//
//             merge(
//                 wire_type,
//                 &mut roundtrip_value,
//                 &mut buf,
//                 DecodeContext::default(),
//             )
//                 .map_err(|error| TestCaseError::fail(error.to_string()))?;
//         }
//
//         prop_assert_eq!(value, roundtrip_value);
//
//         Ok(())
//     }
//
//     #[test]
//     fn string_merge_invalid_utf8() {
//         let mut s = String::new();
//         let buf = b"\x02\x80\x80";
//
//         let r = string::merge(
//             WireType::LengthDelimited,
//             &mut s,
//             &mut &buf[..],
//             DecodeContext::default(),
//         );
//         r.expect_err("must be an error");
//         assert!(s.is_empty());
//     }
//
//     #[test]
//     fn varint() {
//         fn check(value: u64, mut encoded: &[u8]) {
//             // Small buffer.
//             let mut buf = Vec::with_capacity(1);
//             encode_varint(value, &mut buf);
//             assert_eq!(buf, encoded);
//
//             // Large buffer.
//             let mut buf = Vec::with_capacity(100);
//             encode_varint(value, &mut buf);
//             assert_eq!(buf, encoded);
//
//             assert_eq!(encoded_len_varint(value), encoded.len());
//
//             let roundtrip_value =
//                 decode_varint(&mut <&[u8]>::clone(&encoded)).expect("decoding failed");
//             assert_eq!(value, roundtrip_value);
//
//             let roundtrip_value = decode_varint_slow(&mut encoded).expect("slow decoding failed");
//             assert_eq!(value, roundtrip_value);
//         }
//
//         check(2u64.pow(0) - 1, &[0x00]);
//         check(2u64.pow(0), &[0x01]);
//
//         check(2u64.pow(7) - 1, &[0x7F]);
//         check(2u64.pow(7), &[0x80, 0x01]);
//         check(300, &[0xAC, 0x02]);
//
//         check(2u64.pow(14) - 1, &[0xFF, 0x7F]);
//         check(2u64.pow(14), &[0x80, 0x80, 0x01]);
//
//         check(2u64.pow(21) - 1, &[0xFF, 0xFF, 0x7F]);
//         check(2u64.pow(21), &[0x80, 0x80, 0x80, 0x01]);
//
//         check(2u64.pow(28) - 1, &[0xFF, 0xFF, 0xFF, 0x7F]);
//         check(2u64.pow(28), &[0x80, 0x80, 0x80, 0x80, 0x01]);
//
//         check(2u64.pow(35) - 1, &[0xFF, 0xFF, 0xFF, 0xFF, 0x7F]);
//         check(2u64.pow(35), &[0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);
//
//         check(2u64.pow(42) - 1, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F]);
//         check(2u64.pow(42), &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);
//
//         check(
//             2u64.pow(49) - 1,
//             &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
//         );
//         check(
//             2u64.pow(49),
//             &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
//         );
//
//         check(
//             2u64.pow(56) - 1,
//             &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
//         );
//         check(
//             2u64.pow(56),
//             &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
//         );
//
//         check(
//             2u64.pow(63) - 1,
//             &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
//         );
//         check(
//             2u64.pow(63),
//             &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
//         );
//
//         check(
//             u64::MAX,
//             &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
//         );
//     }
//
//     #[test]
//     fn varint_overflow() {
//         let mut u64_max_plus_one: &[u8] =
//             &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x02];
//
//         decode_varint(&mut u64_max_plus_one).expect_err("decoding u64::MAX + 1 succeeded");
//         decode_varint_slow(&mut u64_max_plus_one)
//             .expect_err("slow decoding u64::MAX + 1 succeeded");
//     }
//
//     /// This big bowl o' macro soup generates an encoding property test for each combination of map
//     /// type, scalar map key, and value type.
//     /// TODO: these tests take a long time to compile, can this be improved?
//     #[cfg(feature = "std")]
//     macro_rules! map_tests {
//         (keys: $keys:tt,
//          vals: $vals:tt) => {
//             mod hash_map {
//                 map_tests!(@private HashMap, hash_map, $keys, $vals);
//             }
//             mod btree_map {
//                 map_tests!(@private BTreeMap, btree_map, $keys, $vals);
//             }
//         };
//
//         (@private $map_type:ident,
//                   $mod_name:ident,
//                   [$(($key_ty:ty, $key_proto:ident)),*],
//                   $vals:tt) => {
//             $(
//                 mod $key_proto {
//                     use std::collections::$map_type;
//
//                     use proptest::prelude::*;
//
//                     use crate::encoding::*;
//                     use crate::encoding::test::check_collection_type;
//
//                     map_tests!(@private $map_type, $mod_name, ($key_ty, $key_proto), $vals);
//                 }
//             )*
//         };
//
//         (@private $map_type:ident,
//                   $mod_name:ident,
//                   ($key_ty:ty, $key_proto:ident),
//                   [$(($val_ty:ty, $val_proto:ident)),*]) => {
//             $(
//                 proptest! {
//                     #[test]
//                     fn $val_proto(values: $map_type<$key_ty, $val_ty>, tag in MIN_TAG..=MAX_TAG) {
//                         check_collection_type(values, tag, WireType::LengthDelimited,
//                                               |tag, values, buf| {
//                                                   $mod_name::encode($key_proto::encode,
//                                                                     $key_proto::encoded_len,
//                                                                     $val_proto::encode,
//                                                                     $val_proto::encoded_len,
//                                                                     tag,
//                                                                     values,
//                                                                     buf)
//                                               },
//                                               |wire_type, values, buf, ctx| {
//                                                   check_wire_type(WireType::LengthDelimited, wire_type)?;
//                                                   $mod_name::merge($key_proto::merge,
//                                                                    $val_proto::merge,
//                                                                    values,
//                                                                    buf,
//                                                                    ctx)
//                                               },
//                                               |tag, values| {
//                                                   $mod_name::encoded_len($key_proto::encoded_len,
//                                                                          $val_proto::encoded_len,
//                                                                          tag,
//                                                                          values)
//                                               })?;
//                     }
//                 }
//              )*
//         };
//     }
//
//     #[cfg(feature = "std")]
//     map_tests!(keys: [
//         (i32, int32),
//         (i64, int64),
//         (u32, uint32),
//         (u64, uint64),
//         (i32, sint32),
//         (i64, sint64),
//         (u32, fixed32),
//         (u64, fixed64),
//         (i32, sfixed32),
//         (i64, sfixed64),
//         (bool, bool),
//         (String, string)
//     ],
//     vals: [
//         (f32, float),
//         (f64, double),
//         (i32, int32),
//         (i64, int64),
//         (u32, uint32),
//         (u64, uint64),
//         (i32, sint32),
//         (i64, sint64),
//         (u32, fixed32),
//         (u64, fixed64),
//         (i32, sfixed32),
//         (i64, sfixed64),
//         (bool, bool),
//         (String, string),
//         (Vec<u8>, bytes)
//     ]);
// }
