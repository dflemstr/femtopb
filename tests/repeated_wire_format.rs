//! A `repeated` scalar field must accept BOTH the unpacked and the packed wire formats, since a
//! peer may encode it either way (proto3 packs repeated scalars by default). Regression test for
//! `Repeated` previously rejecting the packed encoding.

use femtopb::Message as _;

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct M<'a> {
    #[femtopb(int32, repeated, tag = 1)]
    pub xs: femtopb::Repeated<'a, i32, femtopb::item_encoding::Int32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

fn decoded(buf: &[u8]) -> Vec<i32> {
    M::decode(buf)
        .unwrap()
        .xs
        .iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[test]
fn repeated_accepts_packed_encoding() {
    // field 1, length-delimited chunk [1,2,3]: key 0x0A, len 3, then varints.
    assert_eq!(decoded(&[0x0A, 0x03, 0x01, 0x02, 0x03]), vec![1, 2, 3]);
}

#[test]
fn repeated_accepts_unpacked_encoding() {
    // field 1 repeated three times as varints: key 0x08 then value, thrice.
    assert_eq!(decoded(&[0x08, 0x01, 0x08, 0x02, 0x08, 0x03]), vec![1, 2, 3]);
}

#[test]
fn repeated_accepts_mixed_packed_and_unpacked() {
    // a packed chunk [1,2] followed by an unpacked 3.
    assert_eq!(decoded(&[0x0A, 0x02, 0x01, 0x02, 0x08, 0x03]), vec![1, 2, 3]);
}

#[test]
fn repeated_empty_packed_chunk_yields_nothing() {
    assert_eq!(decoded(&[0x0A, 0x00]), Vec::<i32>::new());
}
