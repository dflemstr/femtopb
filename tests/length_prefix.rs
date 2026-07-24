//! Regression tests for length-prefix handling in message decoding.

use femtopb::Message as _;

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Tiny<'a> {
    #[femtopb(int32, tag = 1)]
    pub value: i32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[test]
fn decode_length_delimited_rejects_length_beyond_buffer() {
    // The length prefix claims 10 bytes, but only a single content byte follows. This used to
    // panic via `slice::split_at`; it must now surface a clean `BufferUnderflow` error instead.
    let buf = [0x0Au8, 0x08];
    let mut cursor: &[u8] = &buf;
    let err = Tiny::decode_length_delimited::<()>(&mut cursor).unwrap_err();
    assert_eq!(err, femtopb::error::DecodeError::BufferUnderflow);
}

#[test]
fn decode_length_delimited_reads_exactly_one_message() {
    // A well-formed length-delimited `Tiny { value: 1 }` followed by trailing bytes: the decoder
    // should consume exactly the prefixed message and leave the cursor pointing at the remainder.
    let original = Tiny {
        value: 1,
        ..Default::default()
    };
    let mut body = vec![0u8; original.encoded_len()];
    original.encode(&mut body.as_mut_slice()).unwrap();

    let mut buf = vec![u8::try_from(body.len()).unwrap()];
    buf.extend_from_slice(&body);
    buf.extend_from_slice(&[0xDE, 0xAD]); // trailing bytes that must be left untouched

    let mut cursor: &[u8] = &buf;
    let decoded = Tiny::decode_length_delimited::<()>(&mut cursor).unwrap();
    assert_eq!(decoded, original);
    assert_eq!(cursor, &[0xDE, 0xAD]);
}
