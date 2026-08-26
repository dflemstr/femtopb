//! The crate promises that no `femtopb` code panics. `Message::encode` checks the buffer up front
//! and reports an `EncodeError`, but `encode_raw` writes without checking — so it is `encode_raw`
//! that has to stay panic-free when a caller hands it a buffer that is too small. These tests drive
//! every writing code path (varint, fixed-width, length-delimited, lazily-parsed and nested) at
//! every buffer size from empty up to exactly big enough, and assert that each one truncates rather
//! than panicking, and never writes past the end of the buffer.

use femtopb::{item_encoding, packed, repeated, Message as _};

#[derive(Clone, Debug, Default, PartialEq, femtopb::Enumeration)]
#[derive(Copy)]
pub enum Color {
    #[default]
    Red = 0,
    Green = 1,
}

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Leaf<'a> {
    #[femtopb(int32, tag = 1)]
    pub n: i32,
    #[femtopb(string, tag = 2)]
    pub s: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[derive(Clone, Debug, PartialEq, femtopb::Oneof)]
pub enum Kind<'a> {
    #[femtopb(sint64, tag = 20)]
    Signed(i64),
    #[femtopb(bytes, tag = 21)]
    Blob(&'a [u8]),
}

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Everything<'a> {
    #[femtopb(int32, tag = 1)]
    pub varint: i32,
    #[femtopb(sfixed64, tag = 2)]
    pub fixed: i64,
    #[femtopb(double, tag = 3)]
    pub float: f64,
    #[femtopb(string, tag = 4)]
    pub text: &'a str,
    #[femtopb(bytes, tag = 5)]
    pub blob: &'a [u8],
    #[femtopb(enumeration, tag = 6)]
    pub color: femtopb::EnumValue<Color>,
    #[femtopb(message, optional, tag = 7)]
    pub leaf: Option<Leaf<'a>>,
    #[femtopb(int32, packed, tag = 8)]
    pub packed_ints: packed::Packed<'a, i32, item_encoding::Int32>,
    #[femtopb(fixed32, packed, tag = 9)]
    pub packed_fixed: packed::Packed<'a, u32, item_encoding::Fixed32>,
    #[femtopb(string, repeated, tag = 10)]
    pub texts: repeated::Repeated<'a, &'a str, item_encoding::String>,
    #[femtopb(message, repeated, tag = 11)]
    pub leaves: repeated::Repeated<'a, Leaf<'a>, item_encoding::Message<'a, Leaf<'a>>>,
    #[femtopb(enumeration, packed, tag = 12)]
    pub colors: packed::Packed<'a, femtopb::EnumValue<Color>, item_encoding::Enum<Color>>,
    #[femtopb(oneof, tags = [20, 21])]
    pub kind: Option<Kind<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

/// Writes `message` into every buffer size from `0` up to `encoded_len()`, asserting that a short
/// buffer truncates instead of panicking, that nothing is written past the end, that the checked
/// `encode` refuses without touching the buffer, and that an exactly-sized buffer produces the
/// full encoding.
fn assert_truncates_at_every_size<'a, M: femtopb::Message<'a>>(message: &M) {
    let needed = message.encoded_len();
    let mut full = vec![0u8; needed];
    message.encode_raw(&mut full.as_mut_slice());

    for size in 0..needed {
        // A sentinel byte past the end catches any write that runs over.
        let mut buf = vec![0xAAu8; size + 1];
        let mut cursor = &mut buf[..size];
        message.encode_raw(&mut cursor);
        assert_eq!(
            buf[size], 0xAA,
            "encode_raw wrote past the end of a {size}-byte buffer"
        );
        // Truncation is not required to fill the buffer to the brim: a value that does not fit
        // whole (a fixed-width scalar, or a length-delimited payload) is skipped rather than
        // written partially. All that is guaranteed is that nothing lands outside the buffer.

        // The checked API refuses outright and leaves the buffer untouched.
        let mut untouched = vec![0xAAu8; size];
        let err = message
            .encode(&mut untouched.as_mut_slice())
            .expect_err("encode must reject a buffer smaller than encoded_len");
        assert_eq!(err, femtopb::error::EncodeError::new(needed, size));
        assert!(untouched.iter().all(|&b| b == 0xAA));
    }

    // Exactly enough room: the full message, and the cursor lands on the end.
    let mut exact = vec![0u8; needed];
    let mut cursor = exact.as_mut_slice();
    message.encode(&mut cursor).expect("exact-size buffer must succeed");
    assert!(cursor.is_empty());
    assert_eq!(exact, full);
}

fn sample<'a>(leaves: &'a [Leaf<'a>], ints: &'a [i32], texts: &'a [&'a str]) -> Everything<'a> {
    Everything {
        varint: -300,
        fixed: i64::MIN,
        float: core::f64::consts::PI,
        text: "a string long enough to need a multi-byte length prefix and then some",
        blob: b"\x00\x01\x02\xff",
        color: femtopb::EnumValue::Known(Color::Green),
        leaf: Some(Leaf {
            n: 7,
            s: "nested",
            ..Default::default()
        }),
        packed_ints: packed::Packed::from_slice(ints),
        packed_fixed: packed::Packed::empty(),
        texts: repeated::Repeated::from_slice(texts),
        leaves: repeated::Repeated::from_slice(leaves),
        colors: packed::Packed::empty(),
        kind: Some(Kind::Signed(-1234567)),
        ..Default::default()
    }
}

#[test]
fn every_field_kind_truncates_instead_of_panicking() {
    let leaves = [
        Leaf {
            n: 1,
            s: "one",
            ..Default::default()
        },
        Leaf {
            n: 2,
            s: "two",
            ..Default::default()
        },
    ];
    let ints = [1, -2, 300, -400000];
    let texts = ["alpha", "beta", ""];
    assert_truncates_at_every_size(&sample(&leaves, &ints, &texts));
}

#[test]
fn the_bytes_oneof_variant_truncates_too() {
    let leaves: [Leaf; 0] = [];
    let ints: [i32; 0] = [];
    let texts: [&str; 0] = [];
    let mut message = sample(&leaves, &ints, &texts);
    message.kind = Some(Kind::Blob(b"some bytes that will not fit"));
    assert_truncates_at_every_size(&message);
}

#[test]
fn lazily_parsed_fields_backed_by_a_buffer_truncate_too() {
    // Re-encoding a *decoded* message drives the `Repeated`/`Packed` iterators and the unknown-field
    // copier, which are different code paths from the slice-backed fields above.
    let leaves = [Leaf {
        n: 3,
        s: "three",
        ..Default::default()
    }];
    let ints = [5, 6, 7];
    let texts = ["x", "yy"];
    let original = sample(&leaves, &ints, &texts);
    let mut wire = vec![0u8; original.encoded_len()];
    original.encode(&mut wire.as_mut_slice()).unwrap();

    let decoded = Everything::decode(wire.as_slice()).unwrap();
    assert_truncates_at_every_size(&decoded);
}

#[test]
fn unknown_fields_truncate_instead_of_panicking() {
    // A message carrying fields its schema does not declare re-encodes them verbatim; that copy
    // must respect the buffer too.
    let wire = [
        0x08, 0x01, // tag 1 (declared): varint 1
        0x62, 0x03, b'a', b'b', b'c', // tag 12 (unknown): length-delimited "abc"
        0x68, 0x2a, // tag 13 (unknown): varint 42
    ];
    let decoded = Leaf::decode(&wire).unwrap();
    assert_truncates_at_every_size(&decoded);
}

#[test]
fn length_delimited_encoding_truncates_instead_of_panicking() {
    let leaf = Leaf {
        n: 9,
        s: "delimited",
        ..Default::default()
    };
    let needed = leaf.encoded_len() + 1; // one-byte length prefix
    for size in 0..needed {
        let mut buf = vec![0u8; size];
        let err = leaf
            .encode_length_delimited(&mut buf.as_mut_slice())
            .expect_err("a short buffer must be rejected");
        assert_eq!(err, femtopb::error::EncodeError::new(needed, size));
    }
    let mut buf = vec![0u8; needed];
    let mut cursor = buf.as_mut_slice();
    leaf.encode_length_delimited(&mut cursor).unwrap();
    assert!(cursor.is_empty());
    let mut read: &[u8] = &buf;
    assert_eq!(Leaf::decode_length_delimited(&mut read).unwrap(), leaf);
}
