//! Coverage for `femtopb::deferred::Deferred`, which lazily stores a message's bytes and only
//! parses them on demand. The module had no tests.

use femtopb::deferred::Deferred;
use femtopb::Message as _;

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Inner<'a> {
    #[femtopb(int32, tag = 1)]
    pub value: i32,
    #[femtopb(string, tag = 2)]
    pub label: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Outer<'a> {
    #[femtopb(message, optional, tag = 1)]
    pub inner: Option<Deferred<'a, Inner<'a>>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

fn encode(inner: &Inner) -> Vec<u8> {
    let mut buf = vec![0u8; inner.encoded_len()];
    inner.encode(&mut buf.as_mut_slice()).unwrap();
    buf
}

/// Constructs a `Deferred<Inner>` that lazily wraps `bytes` (this is what the `Message::decode`
/// impl does — it stores without parsing).
fn deferred(bytes: &[u8]) -> Deferred<'_, Inner<'_>> {
    <Deferred<Inner> as femtopb::Message>::decode(bytes).unwrap()
}

#[test]
fn decode_stores_bytes_without_parsing() {
    // A buffer that is NOT a valid `Inner` (tag 1 varint key with no value) still wraps
    // successfully — the error only surfaces when `.decode()` actually parses it.
    let malformed: &[u8] = &[0x08];
    let d = deferred(malformed);
    assert_eq!(
        d.decode().unwrap_err(),
        femtopb::error::DecodeError::BufferUnderflow
    );
}

#[test]
fn decode_yields_the_inner_message() {
    let inner = Inner {
        value: 42,
        label: "hi",
        ..Default::default()
    };
    let bytes = encode(&inner);
    assert_eq!(deferred(&bytes).decode().unwrap(), inner);
}

#[test]
fn encoded_len_and_encode_match_the_inner_message() {
    let inner = Inner {
        value: -7,
        label: "abc",
        ..Default::default()
    };
    let bytes = encode(&inner);
    let d = deferred(&bytes);

    // `encoded_len`/`encode_raw` copy the stored bytes, so for a canonically-encoded buffer they
    // reproduce the inner encoding exactly.
    assert_eq!(d.encoded_len(), inner.encoded_len());
    let mut out = vec![0u8; d.encoded_len()];
    d.encode(&mut out.as_mut_slice()).unwrap();
    assert_eq!(out, bytes);
}

#[test]
fn a_deferred_field_is_re_encoded_verbatim() {
    // The point of a deferred field is that it is never parsed, so re-encoding must reproduce the
    // exact bytes rather than a normalised re-encoding of whatever they happen to parse as. Here the
    // two fields appear in descending tag order, which `Inner`'s own encoder would never emit.
    let non_canonical: &[u8] = &[0x12, 0x01, b'x', 0x08, 0x05]; // tag 2 = "x", then tag 1 = 5
    let d = deferred(non_canonical);
    assert_eq!(d.encoded_len(), non_canonical.len());
    let mut out = vec![0u8; d.encoded_len()];
    d.encode(&mut out.as_mut_slice()).unwrap();
    assert_eq!(out, non_canonical);
}

#[test]
fn a_malformed_deferred_is_preserved_rather_than_dropped() {
    // Bytes that do not parse are still carried through: dropping them would silently discard a
    // sub-message the sender did send, and this side never claimed to understand it.
    let malformed: &[u8] = &[0x08]; // undecodable
    let d = deferred(malformed);
    assert!(d.decode().is_err());
    assert_eq!(d.encoded_len(), malformed.len());
    let mut out = vec![0u8; d.encoded_len()];
    d.encode(&mut out.as_mut_slice()).unwrap();
    assert_eq!(out, malformed);
}

#[test]
fn as_bytes_exposes_the_undecoded_buffer() {
    let inner = Inner {
        value: 3,
        label: "z",
        ..Default::default()
    };
    let bytes = encode(&inner);
    assert_eq!(deferred(&bytes).as_bytes(), bytes.as_slice());
}

#[test]
fn deeply_nested_deferred_fields_do_not_overflow_the_stack() {
    // `Deferred` is what `femtopb-build` emits for a directly recursive message field, and decoding
    // is lazy — so a hostile buffer can carry far more nesting than the schema's own type depth.
    // Sizing and writing it back out must not recurse once per level.
    #[derive(Clone, Debug, PartialEq, femtopb::Message)]
    pub struct Rec<'a> {
        #[femtopb(message, optional, tag = 1)]
        pub child: Option<Deferred<'a, Rec<'a>>>,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }

    /// `depth` levels of `key(tag 1, length-delimited), len, <inner>`.
    fn nest(depth: usize) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        for _ in 0..depth {
            let mut next = vec![0x0A];
            let mut len = buf.len() as u64;
            loop {
                if len < 0x80 {
                    next.push(len as u8);
                    break;
                }
                next.push(((len & 0x7F) | 0x80) as u8);
                len >>= 7;
            }
            next.extend_from_slice(&buf);
            buf = next;
        }
        buf
    }

    let wire = nest(100_000);
    let decoded = Rec::decode(&wire).expect("decoding is lazy, so any depth decodes");
    assert_eq!(decoded.encoded_len(), wire.len());

    let mut out = vec![0u8; decoded.encoded_len()];
    decoded.encode(&mut out.as_mut_slice()).unwrap();
    assert_eq!(out, wire);
}

#[test]
fn clear_empties_the_buffer() {
    let inner = Inner {
        value: 1,
        ..Default::default()
    };
    let bytes = encode(&inner);
    let mut d = deferred(&bytes);
    d.clear();
    assert_eq!(d.encoded_len(), 0);
}

#[test]
fn partial_eq_compares_decoded_values_not_bytes() {
    let inner = Inner {
        value: 5,
        label: "x",
        ..Default::default()
    };
    let bytes = encode(&inner);

    // Same logical message encoded with the two fields in the opposite order decodes equal, so the
    // two `Deferred`s must compare equal despite different byte buffers.
    let mut reordered = Vec::new();
    reordered.extend_from_slice(&bytes[bytes.len() - 3..]); // the string field (tag 2)
    reordered.extend_from_slice(&bytes[..bytes.len() - 3]); // the int field (tag 1)
    assert_eq!(deferred(&bytes), deferred(&reordered));

    let other = encode(&Inner {
        value: 6,
        label: "x",
        ..Default::default()
    });
    assert_ne!(deferred(&bytes), deferred(&other));
}

#[test]
fn deferred_as_a_message_field_round_trips_lazily() {
    let inner = Inner {
        value: 99,
        label: "field",
        ..Default::default()
    };
    let inner_bytes = encode(&inner);
    let outer = Outer {
        inner: Some(deferred(&inner_bytes)),
        ..Default::default()
    };

    let mut buf = vec![0u8; outer.encoded_len()];
    outer.encode(&mut buf.as_mut_slice()).unwrap();

    let decoded = Outer::decode(buf.as_slice()).unwrap();
    // The decoded field is a lazily-stored `Deferred`; parsing it yields the original inner message.
    assert_eq!(decoded.inner.unwrap().decode().unwrap(), inner);
}
