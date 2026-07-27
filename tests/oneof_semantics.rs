//! Coverage for oneof decode semantics that the round-trip suites don't reach: the unknown-tag
//! error path, protobuf last-one-wins merge semantics, and clearing.

use femtopb::Message as _;

#[derive(Clone, Debug, PartialEq, femtopb::Oneof)]
pub enum Choice<'a> {
    #[femtopb(int32, tag = 1)]
    Int(i32),
    #[femtopb(string, tag = 2)]
    Text(&'a str),
}

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Msg<'a> {
    #[femtopb(oneof, tags = [1, 2])]
    pub choice: Option<Choice<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[test]
fn oneof_decode_rejects_a_tag_with_no_matching_variant() {
    // The generated `Oneof::decode` has a catch-all that the message decoder never routes to (it
    // only dispatches declared tags), so exercise it directly.
    let mut cursor: &[u8] = &[];
    let err = <Choice as femtopb::oneof::Oneof>::decode(
        99,
        femtopb::encoding::WireType::Varint,
        &[],
        &mut cursor,
    )
    .unwrap_err();
    assert_eq!(err, femtopb::error::DecodeError::UnexpectedTagValue(99));
}

#[test]
fn later_oneof_field_wins() {
    // Two different oneof tags on the wire: tag 1 (int 5) then tag 2 (string "x"). Protobuf
    // semantics keep the last one.
    let buf = [
        0x08, 0x05, // tag 1, varint 5
        0x12, 0x01, b'x', // tag 2, length-delimited "x"
    ];
    let decoded = Msg::decode(&buf).unwrap();
    assert_eq!(decoded.choice, Some(Choice::Text("x")));

    // Reverse the order: tag 2 then tag 1 keeps the int.
    let buf = [0x12, 0x01, b'x', 0x08, 0x05];
    assert_eq!(Msg::decode(&buf).unwrap().choice, Some(Choice::Int(5)));
}

#[test]
fn clear_drops_an_active_oneof() {
    let mut msg = Msg {
        choice: Some(Choice::Int(7)),
        ..Default::default()
    };
    msg.clear();
    assert_eq!(msg.choice, None);
    assert_eq!(msg.encoded_len(), 0);
}

#[test]
fn absent_oneof_round_trips_as_none() {
    let msg = Msg::default();
    let mut buf = vec![0u8; msg.encoded_len()];
    msg.encode(&mut buf.as_mut_slice()).unwrap();
    assert_eq!(Msg::decode(buf.as_slice()).unwrap().choice, None);
}
