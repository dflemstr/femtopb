//! Coverage for `EnumValue` (known/unknown discriminants) and custom field defaults, which the
//! round-trip suites only touch via proptest.

use femtopb::Message as _;

#[derive(Clone, Copy, Debug, Default, PartialEq, femtopb::Enumeration)]
pub enum Color {
    #[default]
    Red = 0,
    Green = 1,
    Blue = 2,
}

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Msg<'a> {
    #[femtopb(enumeration, tag = 1)]
    pub color: femtopb::EnumValue<Color>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Defaulted<'a> {
    #[femtopb(int32, tag = 1, default = 41)]
    pub n: i32,
    #[femtopb(enumeration, tag = 2, default = Color::Green)]
    pub color: femtopb::EnumValue<Color>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[test]
fn to_raw_returns_the_discriminant() {
    assert_eq!(femtopb::EnumValue::Known(Color::Blue).to_raw(), 2);
    assert_eq!(femtopb::EnumValue::<Color>::Unknown(999).to_raw(), 999);
    assert_eq!(femtopb::EnumValue::<Color>::Unknown(-1).to_raw(), -1);
}

#[test]
fn default_is_known_first_variant() {
    assert_eq!(
        femtopb::EnumValue::<Color>::default(),
        femtopb::EnumValue::Known(Color::Red)
    );
}

#[test]
fn decoding_maps_known_and_unknown_discriminants() {
    // tag 1 (varint) with a known discriminant 1 -> Known(Green).
    assert_eq!(
        Msg::decode(&[0x08, 0x01]).unwrap().color,
        femtopb::EnumValue::Known(Color::Green)
    );
    // An unrecognised discriminant 7 -> Unknown(7).
    assert_eq!(
        Msg::decode(&[0x08, 0x07]).unwrap().color,
        femtopb::EnumValue::Unknown(7)
    );
}

#[test]
fn negative_unknown_discriminant_round_trips() {
    let msg = Msg {
        color: femtopb::EnumValue::Unknown(-1),
        ..Default::default()
    };
    let mut buf = vec![0u8; msg.encoded_len()];
    msg.encode(&mut buf.as_mut_slice()).unwrap();
    // A negative i32 sign-extends to a full 10-byte varint (key + 10 bytes).
    assert_eq!(buf.len(), 11);
    assert_eq!(Msg::decode(buf.as_slice()).unwrap(), msg);
}

#[test]
fn custom_defaults_apply_on_decode_and_are_suppressed_on_encode() {
    // An empty buffer leaves both fields at their declared (non-zero) defaults.
    let decoded = Defaulted::decode(&[]).unwrap();
    assert_eq!(decoded.n, 41);
    assert_eq!(decoded.color, femtopb::EnumValue::Known(Color::Green));
    assert_eq!(decoded, Defaulted::default());

    // A message whose fields equal their defaults encodes to nothing.
    assert_eq!(Defaulted::default().encoded_len(), 0);

    // Differing values are encoded and round-trip.
    let changed = Defaulted {
        n: 42,
        color: femtopb::EnumValue::Known(Color::Blue),
        ..Default::default()
    };
    assert!(changed.encoded_len() > 0);
    let mut buf = vec![0u8; changed.encoded_len()];
    changed.encode(&mut buf.as_mut_slice()).unwrap();
    assert_eq!(Defaulted::decode(buf.as_slice()).unwrap(), changed);
}
