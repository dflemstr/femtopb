//! When a packed/repeated field is decoded from a corrupt backing buffer, iteration can yield some
//! `Ok` values followed by an error. The encoders write only the `Ok` values, so `encoded_len` and
//! the length prefix must count only those — otherwise the re-encoded output is inconsistent
//! (overstated length prefix / trailing padding). Regression test for that over-counting.

use femtopb::Message as _;

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Packed<'a> {
    #[femtopb(sfixed32, packed, tag = 1)]
    pub xs: femtopb::Packed<'a, i32, femtopb::item_encoding::SFixed32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[test]
fn packed_corrupt_tail_reencodes_consistently() {
    // field 1, length-delimited chunk of 6 bytes: one 4-byte sfixed32 (= 5) then 2 trailing bytes.
    // Iterating yields Ok(5) then an error.
    let corrupt = [0x0A, 0x06, 5, 0, 0, 0, 0xFF, 0xFF];
    let msg = Packed::decode(&corrupt).unwrap();

    // encoded_len must match exactly the number of bytes encode writes (the encode contract).
    let len = msg.encoded_len();
    let mut buf = vec![0u8; len];
    let remaining_before = buf.len();
    {
        let mut slice = buf.as_mut_slice();
        msg.encode(&mut slice).unwrap();
        // The cursor must be fully consumed: bytes written == encoded_len.
        assert_eq!(slice.len(), 0, "encoded_len ({len}) != bytes written");
    }
    assert_eq!(remaining_before, len);

    // And the re-encoded message must round-trip to exactly the successfully-decoded value.
    let again = Packed::decode(&buf).unwrap();
    let got: Vec<i32> = again.xs.iter().collect::<Result<_, _>>().unwrap();
    assert_eq!(got, vec![5]);
}
