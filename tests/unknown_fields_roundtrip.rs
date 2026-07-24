use femtopb::Message as _;
#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct V2<'a> {
    #[femtopb(int32, tag = 1)] pub a: i32,
    #[femtopb(string, tag = 2)] pub s: &'a str,
    #[femtopb(int32, tag = 3)] pub c: i32,
    #[femtopb(unknown_fields)] pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct V1<'a> {
    #[femtopb(int32, tag = 1)] pub a: i32,
    #[femtopb(unknown_fields)] pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[test]
fn unknown_fields_survive_roundtrip() {
    let v2 = V2 { a: 7, s: "hello", c: 99, ..Default::default() };
    let mut buf = vec![0; v2.encoded_len()];
    v2.encode(&mut buf.as_mut_slice()).unwrap();
    let v1 = V1::decode(buf.as_slice()).unwrap();
    assert_eq!(v1.a, 7);
    let mut rebuf = vec![0; v1.encoded_len()];
    v1.encode(&mut rebuf.as_mut_slice()).unwrap();
    let v2b = V2::decode(rebuf.as_slice()).unwrap();
    assert_eq!(v2b.a, 7);
    assert_eq!(v2b.s, "hello", "unknown string field dropped");
    assert_eq!(v2b.c, 99, "unknown int field dropped");
}
#[test]
fn cleared_message_drops_unknown_fields() {
    let v2 = V2 { a: 1, s: "x", c: 2, ..Default::default() };
    let mut buf = vec![0; v2.encoded_len()]; v2.encode(&mut buf.as_mut_slice()).unwrap();
    let mut v1 = V1::decode(buf.as_slice()).unwrap();
    v1.clear();
    let mut rebuf = vec![0; v1.encoded_len()]; v1.encode(&mut rebuf.as_mut_slice()).unwrap();
    let v2b = V2::decode(rebuf.as_slice()).unwrap();
    assert_eq!(v2b, V2::default());
}

// Schema knowing tags 1 and 3 but NOT 2 or 4 — so unknown fields are interleaved with known ones
// (not just trailing), exercising both the region's start/end bounds and the known-tag skip.
#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Sparse<'a> {
    #[femtopb(int32, tag = 1)] pub a: i32,
    #[femtopb(int32, tag = 3)] pub c: i32,
    #[femtopb(unknown_fields)] pub unknown_fields: femtopb::UnknownFields<'a>,
}

// Full schema with tags 1..=4.
#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Full<'a> {
    #[femtopb(int32, tag = 1)] pub a: i32,
    #[femtopb(int32, tag = 2)] pub b: i32,
    #[femtopb(int32, tag = 3)] pub c: i32,
    #[femtopb(int32, tag = 4)] pub d: i32,
    #[femtopb(unknown_fields)] pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[test]
fn interleaved_unknown_fields_survive_and_known_are_not_duplicated() {
    // Encoded in tag order 1,2,3,4. Under `Sparse`, tags 2 and 4 are unknown and 3 sits between
    // them (a known field inside the unknown region), while 1 is a known prefix.
    let full = Full { a: 10, b: 20, c: 30, d: 40, ..Default::default() };
    let mut buf = vec![0; full.encoded_len()];
    full.encode(&mut buf.as_mut_slice()).unwrap();

    let sparse = Sparse::decode(buf.as_slice()).unwrap();
    assert_eq!((sparse.a, sparse.c), (10, 30));

    let mut rebuf = vec![0; sparse.encoded_len()];
    sparse.encode(&mut rebuf.as_mut_slice()).unwrap();
    let full_again = Full::decode(rebuf.as_slice()).unwrap();

    // Known fields come through from the struct; unknown 2 and 4 are preserved; nothing duplicated.
    assert_eq!(full_again, full);
}
