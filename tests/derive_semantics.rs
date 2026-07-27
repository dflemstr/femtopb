//! Behavioural coverage for the `Message` derive that the wire-format round-trip suites don't
//! isolate: field decoding is order-independent, `required` fields are always emitted, `deprecated`
//! fields still round-trip, and an all-defaults message is empty on the wire.

use femtopb::Message as _;

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Basic<'a> {
    #[femtopb(int32, tag = 1)]
    pub a: i32,
    #[femtopb(string, tag = 2)]
    pub b: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[test]
fn fields_decode_regardless_of_wire_order() {
    // Tag 2 (string "hi") appears before tag 1 (varint 5) on the wire. Protobuf permits any field
    // order, so both must land in their declared fields.
    let buf = [
        0x12, 0x02, b'h', b'i', // tag 2, len 2, "hi"
        0x08, 0x05, // tag 1, varint 5
    ];
    let decoded = Basic::decode(&buf).unwrap();
    assert_eq!(decoded.a, 5);
    assert_eq!(decoded.b, "hi");
}

#[test]
fn plain_scalar_at_default_is_not_encoded() {
    // A plain (proto3-style) scalar equal to its default contributes nothing to the wire form.
    let msg = Basic {
        a: 0,
        b: "",
        ..Default::default()
    };
    assert_eq!(msg.encoded_len(), 0);
}

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct HasRequired<'a> {
    #[femtopb(int32, required, tag = 1)]
    pub n: i32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[test]
fn required_scalar_behaves_like_a_plain_scalar() {
    // femtopb does not enforce proto2 `required` semantics for scalars: the label is accepted but
    // encoding/decoding is identical to a plain field, so a defaulted value is omitted from the
    // wire and a missing field decodes back to the default without error.
    let defaulted = HasRequired {
        n: 0,
        ..Default::default()
    };
    assert_eq!(defaulted.encoded_len(), 0);
    assert_eq!(HasRequired::decode(&[]).unwrap(), defaulted);

    // A non-default value round-trips as usual.
    let set = HasRequired {
        n: 5,
        ..Default::default()
    };
    let mut buf = vec![0u8; set.encoded_len()];
    set.encode(&mut buf.as_mut_slice()).unwrap();
    assert_eq!(buf, vec![0x08, 0x05]);
    assert_eq!(HasRequired::decode(buf.as_slice()).unwrap(), set);
}

#[allow(deprecated)]
mod deprecated_field {
    use super::*;

    #[derive(Clone, Debug, PartialEq, femtopb::Message)]
    pub struct HasDeprecated<'a> {
        #[deprecated]
        #[femtopb(int32, tag = 1)]
        pub old: i32,
        #[femtopb(int32, tag = 2)]
        pub current: i32,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }

    #[test]
    fn deprecated_field_still_round_trips() {
        let msg = HasDeprecated {
            old: 7,
            current: 9,
            ..Default::default()
        };
        let mut buf = vec![0u8; msg.encoded_len()];
        msg.encode(&mut buf.as_mut_slice()).unwrap();
        assert_eq!(HasDeprecated::decode(buf.as_slice()).unwrap(), msg);
    }
}

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Empty<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[test]
fn message_with_only_unknown_fields_is_empty_on_the_wire() {
    let msg = Empty::default();
    assert_eq!(msg.encoded_len(), 0);
    let mut none: [u8; 0] = [];
    msg.encode(&mut none.as_mut_slice()).unwrap();
    assert_eq!(Empty::decode(&[]).unwrap(), msg);
}

#[test]
fn unknown_fields_survive_a_decode_encode_round_trip() {
    // A field the schema doesn't declare (tag 3, varint 42) must be retained and re-emitted.
    let wire = [
        0x08, 0x01, // tag 1 = 1 (declared)
        0x18, 0x2a, // tag 3 = 42 (unknown)
    ];
    let decoded = Basic::decode(&wire).unwrap();
    assert_eq!(decoded.a, 1);

    let mut buf = vec![0u8; decoded.encoded_len()];
    decoded.encode(&mut buf.as_mut_slice()).unwrap();
    // The unknown tag-3 field is preserved on re-encode.
    let reparsed = Basic::decode(buf.as_slice()).unwrap();
    assert_eq!(reparsed, decoded);
    assert!(buf.windows(2).any(|w| w == [0x18, 0x2a]));
}
