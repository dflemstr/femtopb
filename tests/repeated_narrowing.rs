//! A repeated/packed field whose occurrences come after many other fields must still decode all of
//! its values correctly, even though its retained buffer is now narrowed to start at its first
//! occurrence (skipping the preceding fields). Also exercises re-iterating the same field.

use femtopb::Message as _;

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct M<'a> {
    #[femtopb(int32, tag = 1)] pub a: i32,
    #[femtopb(string, tag = 2)] pub b: &'a str,
    #[femtopb(int32, tag = 3)] pub c: i32,
    #[femtopb(int32, packed, tag = 4)]
    pub packed: femtopb::Packed<'a, i32, femtopb::item_encoding::Int32>,
    #[femtopb(sint64, repeated, tag = 5)]
    pub repeated: femtopb::Repeated<'a, i64, femtopb::item_encoding::SInt64>,
    #[femtopb(unknown_fields)] pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[test]
fn late_repeated_and_packed_fields_decode_after_narrowing() {
    let original = M {
        a: 11,
        b: "hello",
        c: 22,
        packed: femtopb::Packed::from_slice(&[1, 2, 3, 4]),
        repeated: femtopb::Repeated::from_slice(&[-1, -2, -3]),
        ..Default::default()
    };
    let mut buf = vec![0; original.encoded_len()];
    original.encode(&mut buf.as_mut_slice()).unwrap();

    let decoded = M::decode(buf.as_slice()).unwrap();
    assert_eq!((decoded.a, decoded.b, decoded.c), (11, "hello", 22));

    // Iterate each lazily-parsed field more than once (len + collect) to exercise re-scanning from
    // the narrowed start.
    assert_eq!(decoded.packed.len(), 4);
    assert_eq!(
        decoded.packed.iter().collect::<Result<Vec<_>, _>>().unwrap(),
        vec![1, 2, 3, 4]
    );
    assert_eq!(decoded.repeated.len(), 3);
    assert_eq!(
        decoded.repeated.iter().collect::<Result<Vec<_>, _>>().unwrap(),
        vec![-1, -2, -3]
    );

    // Full round-trip equality as a belt-and-braces check.
    assert_eq!(decoded, original);
}
