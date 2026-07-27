//! Coverage for the `Message` trait's encode helpers and the `EncodeError` surface, which the
//! round-trip suites exercise only on the happy path.

use femtopb::Message as _;

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct M<'a> {
    #[femtopb(int32, tag = 1)]
    pub value: i32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[test]
fn encode_into_too_small_buffer_reports_required_and_remaining() {
    let m = M {
        value: 150,
        ..Default::default()
    }; // encodes to [0x08, 0x96, 0x01] — three bytes
    assert_eq!(m.encoded_len(), 3);

    let mut small = [0u8; 2];
    let mut cursor: &mut [u8] = &mut small;
    let err = m.encode(&mut cursor).unwrap_err();
    assert_eq!(err, femtopb::error::EncodeError::new(3, 2));
    // On failure the buffer is left untouched (encode_raw is never reached).
    assert_eq!(cursor.len(), 2);
}

#[test]
fn encode_advances_the_cursor_past_the_message() {
    let m = M {
        value: 150,
        ..Default::default()
    };
    let mut buf = [0u8; 5];
    let mut cursor: &mut [u8] = &mut buf;
    m.encode(&mut cursor).unwrap();
    assert_eq!(cursor.len(), 2); // 5 - 3 written; cursor points at the remaining space
    assert_eq!(&buf[..3], &[0x08, 0x96, 0x01]);
}

#[test]
fn empty_message_encodes_to_nothing() {
    let m = M::default(); // value == 0 (the default) is not encoded
    assert_eq!(m.encoded_len(), 0);
    let mut none: [u8; 0] = [];
    let mut cursor: &mut [u8] = &mut none;
    m.encode(&mut cursor).unwrap();
}

#[test]
fn encode_length_delimited_writes_prefix_then_body() {
    let m = M {
        value: 150,
        ..Default::default()
    };
    let mut buf = vec![0u8; m.encoded_len() + 1];
    let mut cursor: &mut [u8] = &mut buf;
    m.encode_length_delimited(&mut cursor).unwrap();
    assert!(cursor.is_empty());
    // Length prefix 3 (a single varint byte) followed by the three body bytes.
    assert_eq!(buf, vec![0x03, 0x08, 0x96, 0x01]);

    // And it round-trips through the delimited decoder.
    let mut read: &[u8] = &buf;
    assert_eq!(M::decode_length_delimited(&mut read).unwrap(), m);
    assert!(read.is_empty());
}

#[test]
fn encode_length_delimited_too_small_reports_prefix_inclusive_requirement() {
    let m = M {
        value: 150,
        ..Default::default()
    };
    // Body is 3 bytes; with the 1-byte length prefix the requirement is 4.
    let mut small = [0u8; 3];
    let mut cursor: &mut [u8] = &mut small;
    let err = m.encode_length_delimited(&mut cursor).unwrap_err();
    assert_eq!(err, femtopb::error::EncodeError::new(4, 3));
}

#[test]
fn multiple_delimited_messages_chain_in_one_buffer() {
    let first = M {
        value: 1,
        ..Default::default()
    };
    let second = M {
        value: 300,
        ..Default::default()
    };
    let mut buf = vec![0u8; first.encoded_len() + 1 + second.encoded_len() + 1];
    let mut cursor: &mut [u8] = &mut buf;
    first.encode_length_delimited(&mut cursor).unwrap();
    second.encode_length_delimited(&mut cursor).unwrap();

    let mut read: &[u8] = &buf;
    assert_eq!(M::decode_length_delimited(&mut read).unwrap(), first);
    assert_eq!(M::decode_length_delimited(&mut read).unwrap(), second);
    assert!(read.is_empty());
}
