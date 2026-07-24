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
