//! Round-trip and wire-format tests for zig-zag encoded signed varints (`sint32`/`sint64`).
//!
//! The zig-zag transform (`(n << 1) ^ (n >> width-1)`) is the subtle part of signed-varint
//! encoding; sign handling at the `i32`/`i64` extremes is easy to get wrong, so exercise the
//! boundary values explicitly rather than relying only on the proptest suite.

use femtopb::Message as _;

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Signed<'a> {
    #[femtopb(sint32, tag = 1)]
    pub a: i32,
    #[femtopb(sint64, tag = 2)]
    pub b: i64,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

fn round_trip(a: i32, b: i64) {
    let original = Signed {
        a,
        b,
        ..Default::default()
    };
    let mut buf = vec![0u8; original.encoded_len()];
    original.encode(&mut buf.as_mut_slice()).unwrap();
    let decoded = Signed::decode(buf.as_slice()).unwrap();
    assert_eq!(decoded, original, "round-trip of sint32={a}, sint64={b}");
}

#[test]
fn sint_round_trips_boundary_values() {
    for a in [0i32, 1, -1, i32::MIN, i32::MAX, -2, 2] {
        for b in [0i64, 1, -1, i64::MIN, i64::MAX, i64::from(i32::MIN)] {
            round_trip(a, b);
        }
    }
}

#[test]
fn zigzag_wire_encoding_is_correct() {
    // zig-zag maps 0->0, -1->1, 1->2, -2->3, 2->4, ...  so `sint32 a = -1` encodes as a single
    // varint byte `0x01` after its key, and `a = 1` as `0x02`.
    let key = 1u8 << 3; // tag 1, wire type Varint (0)
    for (value, zigzag_byte) in [(-1i32, 0x01u8), (1, 0x02), (-2, 0x03), (2, 0x04)] {
        let msg = Signed {
            a: value,
            ..Default::default()
        };
        let mut buf = vec![0u8; msg.encoded_len()];
        msg.encode(&mut buf.as_mut_slice()).unwrap();
        assert_eq!(buf, vec![key, zigzag_byte], "sint32 = {value}");
    }
}
