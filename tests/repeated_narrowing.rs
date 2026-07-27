//! A repeated/packed field whose occurrences come after many other fields must still decode all of
//! its values correctly, even though its retained buffer is now narrowed to the span from its first
//! occurrence to its last (skipping the fields before *and* after). Also exercises re-iterating the
//! same field.

use femtopb::Message as _;

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct M<'a> {
    #[femtopb(int32, tag = 1)]
    pub a: i32,
    #[femtopb(string, tag = 2)]
    pub b: &'a str,
    #[femtopb(int32, tag = 3)]
    pub c: i32,
    #[femtopb(int32, packed, tag = 4)]
    pub packed: femtopb::Packed<'a, i32, femtopb::item_encoding::Int32>,
    #[femtopb(sint64, repeated, tag = 5)]
    pub repeated: femtopb::Repeated<'a, i64, femtopb::item_encoding::SInt64>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
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
        decoded
            .packed
            .iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![1, 2, 3, 4]
    );
    assert_eq!(decoded.repeated.len(), 3);
    assert_eq!(
        decoded
            .repeated
            .iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![-1, -2, -3]
    );

    // Full round-trip equality as a belt-and-braces check.
    assert_eq!(decoded, original);
}

/// The lazy fields here are followed by trailing scalar fields (higher tags), so their retained
/// window is narrowed on *both* ends — it must stop at the last occurrence and not run into the
/// trailing fields. Exercises the end-bound narrowing (`extend_region`).
#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Surrounded<'a> {
    #[femtopb(int32, tag = 1)]
    pub before: i32,
    #[femtopb(sint64, repeated, tag = 2)]
    pub repeated: femtopb::Repeated<'a, i64, femtopb::item_encoding::SInt64>,
    #[femtopb(int32, packed, tag = 3)]
    pub packed: femtopb::Packed<'a, i32, femtopb::item_encoding::Int32>,
    #[femtopb(int32, tag = 4)]
    pub after: i32,
    #[femtopb(string, tag = 5)]
    pub also_after: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[test]
fn lazy_fields_with_trailing_fields_narrow_on_both_ends() {
    let original = Surrounded {
        before: 7,
        repeated: femtopb::Repeated::from_slice(&[-10, -20, -30]),
        packed: femtopb::Packed::from_slice(&[100, 200, 300]),
        after: 42,
        also_after: "trailing",
        ..Default::default()
    };
    let mut buf = vec![0; original.encoded_len()];
    original.encode(&mut buf.as_mut_slice()).unwrap();

    let decoded = Surrounded::decode(buf.as_slice()).unwrap();

    // The trailing scalars decode correctly (they are not swallowed by the lazy fields' windows).
    assert_eq!(
        (decoded.before, decoded.after, decoded.also_after),
        (7, 42, "trailing")
    );

    // The lazy fields yield exactly their own values and stop before the trailing fields — checked
    // by iterating twice (len re-scans, then collect re-scans) over the doubly-narrowed window.
    assert_eq!(decoded.repeated.len(), 3);
    assert_eq!(
        decoded
            .repeated
            .iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![-10, -20, -30]
    );
    assert_eq!(decoded.packed.len(), 3);
    assert_eq!(
        decoded
            .packed
            .iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![100, 200, 300]
    );

    assert_eq!(decoded, original);
}
