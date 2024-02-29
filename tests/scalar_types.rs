use femtopb::packed;
use femtopb::repeated;

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct ScalarTypes<'a> {
    #[femtopb(int32, tag = 1)]
    pub int32: i32,
    #[femtopb(int64, tag = 2)]
    pub int64: i64,
    #[femtopb(uint32, tag = 3)]
    pub uint32: u32,
    #[femtopb(uint64, tag = 4)]
    pub uint64: u64,
    #[femtopb(sint32, tag = 5)]
    pub sint32: i32,
    #[femtopb(sint64, tag = 6)]
    pub sint64: i64,
    #[femtopb(fixed32, tag = 7)]
    pub fixed32: u32,
    #[femtopb(fixed64, tag = 8)]
    pub fixed64: u64,
    #[femtopb(sfixed32, tag = 9)]
    pub sfixed32: i32,
    #[femtopb(sfixed64, tag = 10)]
    pub sfixed64: i64,
    #[femtopb(float, tag = 11)]
    pub float: f32,
    #[femtopb(double, tag = 12)]
    pub double: f64,
    #[femtopb(bool, tag = 13)]
    pub _bool: bool,
    #[femtopb(string, tag = 14)]
    pub string: &'a str,
    #[femtopb(bytes, tag = 15)]
    pub bytes: &'a [u8],

    #[femtopb(int32, required, tag = 101)]
    pub required_int32: i32,
    #[femtopb(int64, required, tag = 102)]
    pub required_int64: i64,
    #[femtopb(uint32, required, tag = 103)]
    pub required_uint32: u32,
    #[femtopb(uint64, required, tag = 104)]
    pub required_uint64: u64,
    #[femtopb(sint32, required, tag = 105)]
    pub required_sint32: i32,
    #[femtopb(sint64, required, tag = 106)]
    pub required_sint64: i64,
    #[femtopb(fixed32, required, tag = 107)]
    pub required_fixed32: u32,
    #[femtopb(fixed64, required, tag = 108)]
    pub required_fixed64: u64,
    #[femtopb(sfixed32, required, tag = 109)]
    pub required_sfixed32: i32,
    #[femtopb(sfixed64, required, tag = 110)]
    pub required_sfixed64: i64,
    #[femtopb(float, required, tag = 111)]
    pub required_float: f32,
    #[femtopb(double, required, tag = 112)]
    pub required_double: f64,
    #[femtopb(bool, required, tag = 113)]
    pub required_bool: bool,
    #[femtopb(string, required, tag = 114)]
    pub required_string: &'a str,
    #[femtopb(bytes, required, tag = 115)]
    pub required_bytes: &'a [u8],

    #[femtopb(int32, optional, tag = 201)]
    pub optional_int32: Option<i32>,
    #[femtopb(int64, optional, tag = 202)]
    pub optional_int64: Option<i64>,
    #[femtopb(uint32, optional, tag = 203)]
    pub optional_uint32: Option<u32>,
    #[femtopb(uint64, optional, tag = 204)]
    pub optional_uint64: Option<u64>,
    #[femtopb(sint32, optional, tag = 205)]
    pub optional_sint32: Option<i32>,
    #[femtopb(sint64, optional, tag = 206)]
    pub optional_sint64: Option<i64>,
    #[femtopb(fixed32, optional, tag = 207)]
    pub optional_fixed32: Option<u32>,
    #[femtopb(fixed64, optional, tag = 208)]
    pub optional_fixed64: Option<u64>,
    #[femtopb(sfixed32, optional, tag = 209)]
    pub optional_sfixed32: Option<i32>,
    #[femtopb(sfixed64, optional, tag = 210)]
    pub optional_sfixed64: Option<i64>,
    #[femtopb(float, optional, tag = 211)]
    pub optional_float: Option<f32>,
    #[femtopb(double, optional, tag = 212)]
    pub optional_double: Option<f64>,
    #[femtopb(bool, optional, tag = 213)]
    pub optional_bool: Option<bool>,
    #[femtopb(string, optional, tag = 214)]
    pub optional_string: Option<&'a str>,
    #[femtopb(bytes, optional, tag = 215)]
    pub optional_bytes: Option<&'a [u8]>,

    #[femtopb(int32, repeated, tag = 301)]
    pub repeated_int32: femtopb::Repeated<'a, i32, femtopb::item_encoding::Int32>,
    #[femtopb(int64, repeated, tag = 302)]
    pub repeated_int64: femtopb::Repeated<'a, i64, femtopb::item_encoding::Int64>,
    #[femtopb(uint32, repeated, tag = 303)]
    pub repeated_uint32: femtopb::Repeated<'a, u32, femtopb::item_encoding::UInt32>,
    #[femtopb(uint64, repeated, tag = 304)]
    pub repeated_uint64: femtopb::Repeated<'a, u64, femtopb::item_encoding::UInt64>,
    #[femtopb(sint32, repeated, tag = 305)]
    pub repeated_sint32: femtopb::Repeated<'a, i32, femtopb::item_encoding::SInt32>,
    #[femtopb(sint64, repeated, tag = 306)]
    pub repeated_sint64: femtopb::Repeated<'a, i64, femtopb::item_encoding::SInt64>,
    #[femtopb(fixed32, repeated, tag = 307)]
    pub repeated_fixed32: femtopb::Repeated<'a, u32, femtopb::item_encoding::Fixed32>,
    #[femtopb(fixed64, repeated, tag = 308)]
    pub repeated_fixed64: femtopb::Repeated<'a, u64, femtopb::item_encoding::Fixed64>,
    #[femtopb(sfixed32, repeated, tag = 309)]
    pub repeated_sfixed32: femtopb::Repeated<'a, i32, femtopb::item_encoding::SFixed32>,
    #[femtopb(sfixed64, repeated, tag = 310)]
    pub repeated_sfixed64: femtopb::Repeated<'a, i64, femtopb::item_encoding::SFixed64>,
    #[femtopb(float, repeated, tag = 311)]
    pub repeated_float: femtopb::Repeated<'a, f32, femtopb::item_encoding::Float>,
    #[femtopb(double, repeated, tag = 312)]
    pub repeated_double: femtopb::Repeated<'a, f64, femtopb::item_encoding::Double>,
    #[femtopb(bool, repeated, tag = 313)]
    pub repeated_bool: femtopb::Repeated<'a, bool, femtopb::item_encoding::Bool>,
    #[femtopb(string, repeated, tag = 314)]
    pub repeated_string: femtopb::Repeated<'a, &'a str, femtopb::item_encoding::String>,
    #[femtopb(bytes, repeated, tag = 315)]
    pub repeated_bytes: femtopb::Repeated<'a, &'a [u8], femtopb::item_encoding::Bytes>,

    #[femtopb(int32, packed, tag = 401)]
    pub packed_int32: femtopb::Packed<'a, i32, femtopb::item_encoding::Int32>,
    #[femtopb(int64, packed, tag = 402)]
    pub packed_int64: femtopb::Packed<'a, i64, femtopb::item_encoding::Int64>,
    #[femtopb(uint32, packed, tag = 403)]
    pub packed_uint32: femtopb::Packed<'a, u32, femtopb::item_encoding::UInt32>,
    #[femtopb(uint64, packed, tag = 404)]
    pub packed_uint64: femtopb::Packed<'a, u64, femtopb::item_encoding::UInt64>,
    #[femtopb(sint32, packed, tag = 405)]
    pub packed_sint32: femtopb::Packed<'a, i32, femtopb::item_encoding::SInt32>,
    #[femtopb(sint64, packed, tag = 406)]
    pub packed_sint64: femtopb::Packed<'a, i64, femtopb::item_encoding::SInt64>,
    #[femtopb(fixed32, packed, tag = 407)]
    pub packed_fixed32: femtopb::Packed<'a, u32, femtopb::item_encoding::Fixed32>,
    #[femtopb(fixed64, packed, tag = 408)]
    pub packed_fixed64: femtopb::Packed<'a, u64, femtopb::item_encoding::Fixed64>,
    #[femtopb(sfixed32, packed, tag = 409)]
    pub packed_sfixed32: femtopb::Packed<'a, i32, femtopb::item_encoding::SFixed32>,
    #[femtopb(sfixed64, packed, tag = 410)]
    pub packed_sfixed64: femtopb::Packed<'a, i64, femtopb::item_encoding::SFixed64>,
    #[femtopb(float, packed, tag = 411)]
    pub packed_float: femtopb::Packed<'a, f32, femtopb::item_encoding::Float>,
    #[femtopb(double, packed, tag = 412)]
    pub packed_double: femtopb::Packed<'a, f64, femtopb::item_encoding::Double>,
    #[femtopb(bool, packed, tag = 413)]
    pub packed_bool: femtopb::Packed<'a, bool, femtopb::item_encoding::Bool>,
    #[femtopb(string, repeated, tag = 414)]
    pub packed_string: femtopb::Repeated<'a, &'a str, femtopb::item_encoding::String>,
    #[femtopb(bytes, repeated, tag = 415)]
    pub packed_bytes: femtopb::Repeated<'a, &'a [u8], femtopb::item_encoding::Bytes>,

    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ProstScalarTypes {
    #[prost(int32, tag = "001")]
    pub int32: i32,
    #[prost(int64, tag = "002")]
    pub int64: i64,
    #[prost(uint32, tag = "003")]
    pub uint32: u32,
    #[prost(uint64, tag = "004")]
    pub uint64: u64,
    #[prost(sint32, tag = "005")]
    pub sint32: i32,
    #[prost(sint64, tag = "006")]
    pub sint64: i64,
    #[prost(fixed32, tag = "007")]
    pub fixed32: u32,
    #[prost(fixed64, tag = "008")]
    pub fixed64: u64,
    #[prost(sfixed32, tag = "009")]
    pub sfixed32: i32,
    #[prost(sfixed64, tag = "010")]
    pub sfixed64: i64,
    #[prost(float, tag = "011")]
    pub float: f32,
    #[prost(double, tag = "012")]
    pub double: f64,
    #[prost(bool, tag = "013")]
    pub _bool: bool,
    #[prost(string, tag = "014")]
    pub string: String,
    #[prost(bytes = "bytes", tag = "015")]
    pub bytes: bytes::Bytes,

    #[prost(int32, required, tag = "101")]
    pub required_int32: i32,
    #[prost(int64, required, tag = "102")]
    pub required_int64: i64,
    #[prost(uint32, required, tag = "103")]
    pub required_uint32: u32,
    #[prost(uint64, required, tag = "104")]
    pub required_uint64: u64,
    #[prost(sint32, required, tag = "105")]
    pub required_sint32: i32,
    #[prost(sint64, required, tag = "106")]
    pub required_sint64: i64,
    #[prost(fixed32, required, tag = "107")]
    pub required_fixed32: u32,
    #[prost(fixed64, required, tag = "108")]
    pub required_fixed64: u64,
    #[prost(sfixed32, required, tag = "109")]
    pub required_sfixed32: i32,
    #[prost(sfixed64, required, tag = "110")]
    pub required_sfixed64: i64,
    #[prost(float, required, tag = "111")]
    pub required_float: f32,
    #[prost(double, required, tag = "112")]
    pub required_double: f64,
    #[prost(bool, required, tag = "113")]
    pub required_bool: bool,
    #[prost(string, required, tag = "114")]
    pub required_string: String,
    #[prost(bytes = "bytes", required, tag = "115")]
    pub required_bytes: bytes::Bytes,

    #[prost(int32, optional, tag = "201")]
    pub optional_int32: Option<i32>,
    #[prost(int64, optional, tag = "202")]
    pub optional_int64: Option<i64>,
    #[prost(uint32, optional, tag = "203")]
    pub optional_uint32: Option<u32>,
    #[prost(uint64, optional, tag = "204")]
    pub optional_uint64: Option<u64>,
    #[prost(sint32, optional, tag = "205")]
    pub optional_sint32: Option<i32>,
    #[prost(sint64, optional, tag = "206")]
    pub optional_sint64: Option<i64>,

    #[prost(fixed32, optional, tag = "207")]
    pub optional_fixed32: Option<u32>,
    #[prost(fixed64, optional, tag = "208")]
    pub optional_fixed64: Option<u64>,
    #[prost(sfixed32, optional, tag = "209")]
    pub optional_sfixed32: Option<i32>,
    #[prost(sfixed64, optional, tag = "210")]
    pub optional_sfixed64: Option<i64>,
    #[prost(float, optional, tag = "211")]
    pub optional_float: Option<f32>,
    #[prost(double, optional, tag = "212")]
    pub optional_double: Option<f64>,
    #[prost(bool, optional, tag = "213")]
    pub optional_bool: Option<bool>,
    #[prost(string, optional, tag = "214")]
    pub optional_string: Option<String>,
    #[prost(bytes = "bytes", optional, tag = "215")]
    pub optional_bytes: Option<bytes::Bytes>,

    #[prost(int32, repeated, packed = "false", tag = "301")]
    pub repeated_int32: Vec<i32>,
    #[prost(int64, repeated, packed = "false", tag = "302")]
    pub repeated_int64: Vec<i64>,
    #[prost(uint32, repeated, packed = "false", tag = "303")]
    pub repeated_uint32: Vec<u32>,
    #[prost(uint64, repeated, packed = "false", tag = "304")]
    pub repeated_uint64: Vec<u64>,
    #[prost(sint32, repeated, packed = "false", tag = "305")]
    pub repeated_sint32: Vec<i32>,
    #[prost(sint64, repeated, packed = "false", tag = "306")]
    pub repeated_sint64: Vec<i64>,
    #[prost(fixed32, repeated, packed = "false", tag = "307")]
    pub repeated_fixed32: Vec<u32>,
    #[prost(fixed64, repeated, packed = "false", tag = "308")]
    pub repeated_fixed64: Vec<u64>,
    #[prost(sfixed32, repeated, packed = "false", tag = "309")]
    pub repeated_sfixed32: Vec<i32>,
    #[prost(sfixed64, repeated, packed = "false", tag = "310")]
    pub repeated_sfixed64: Vec<i64>,
    #[prost(float, repeated, packed = "false", tag = "311")]
    pub repeated_float: Vec<f32>,
    #[prost(double, repeated, packed = "false", tag = "312")]
    pub repeated_double: Vec<f64>,
    #[prost(bool, repeated, packed = "false", tag = "313")]
    pub repeated_bool: Vec<bool>,
    #[prost(string, repeated, packed = "false", tag = "314")]
    pub repeated_string: Vec<String>,
    #[prost(bytes = "bytes", repeated, packed = "false", tag = "315")]
    pub repeated_bytes: Vec<bytes::Bytes>,

    #[prost(int32, repeated, tag = "401")]
    pub packed_int32: Vec<i32>,
    #[prost(int64, repeated, tag = "402")]
    pub packed_int64: Vec<i64>,
    #[prost(uint32, repeated, tag = "403")]
    pub packed_uint32: Vec<u32>,
    #[prost(uint64, repeated, tag = "404")]
    pub packed_uint64: Vec<u64>,
    #[prost(sint32, repeated, tag = "405")]
    pub packed_sint32: Vec<i32>,
    #[prost(sint64, repeated, tag = "406")]
    pub packed_sint64: Vec<i64>,
    #[prost(fixed32, repeated, tag = "407")]
    pub packed_fixed32: Vec<u32>,

    #[prost(fixed64, repeated, tag = "408")]
    pub packed_fixed64: Vec<u64>,
    #[prost(sfixed32, repeated, tag = "409")]
    pub packed_sfixed32: Vec<i32>,
    #[prost(sfixed64, repeated, tag = "410")]
    pub packed_sfixed64: Vec<i64>,
    #[prost(float, repeated, tag = "411")]
    pub packed_float: Vec<f32>,
    #[prost(double, repeated, tag = "412")]
    pub packed_double: Vec<f64>,
    #[prost(bool, repeated, tag = "413")]
    pub packed_bool: Vec<bool>,
    #[prost(string, repeated, tag = "414")]
    pub packed_string: Vec<String>,
    #[prost(bytes = "bytes", repeated, tag = "415")]
    pub packed_bytes: Vec<bytes::Bytes>,
}

// Typing this out manually to avoid bugs in any sort of automation like macros, etc...
#[derive(Clone, Debug, proptest_derive::Arbitrary)]
pub struct ArbitraryScalarTypes {
    pub int32: i32,
    pub int64: i64,
    pub uint32: u32,
    pub uint64: u64,
    pub sint32: i32,
    pub sint64: i64,
    pub fixed32: u32,
    pub fixed64: u64,
    pub sfixed32: i32,
    pub sfixed64: i64,
    pub float: f32,
    pub double: f64,
    pub _bool: bool,
    pub string: String,
    pub bytes: Vec<u8>,

    pub required_int32: i32,
    pub required_int64: i64,
    pub required_uint32: u32,
    pub required_uint64: u64,
    pub required_sint32: i32,
    pub required_sint64: i64,
    pub required_fixed32: u32,
    pub required_fixed64: u64,
    pub required_sfixed32: i32,
    pub required_sfixed64: i64,
    pub required_float: f32,
    pub required_double: f64,
    pub required_bool: bool,
    pub required_string: String,
    pub required_bytes: Vec<u8>,

    #[proptest(filter = "|&v| v != Some(0)")]
    pub optional_int32: Option<i32>,
    #[proptest(filter = "|&v| v != Some(0)")]
    pub optional_int64: Option<i64>,
    #[proptest(filter = "|&v| v != Some(0)")]
    pub optional_uint32: Option<u32>,
    #[proptest(filter = "|&v| v != Some(0)")]
    pub optional_uint64: Option<u64>,
    #[proptest(filter = "|&v| v != Some(0)")]
    pub optional_sint32: Option<i32>,
    #[proptest(filter = "|&v| v != Some(0)")]
    pub optional_sint64: Option<i64>,
    #[proptest(filter = "|&v| v != Some(0)")]
    pub optional_fixed32: Option<u32>,
    #[proptest(filter = "|&v| v != Some(0)")]
    pub optional_fixed64: Option<u64>,
    #[proptest(filter = "|&v| v != Some(0)")]
    pub optional_sfixed32: Option<i32>,
    #[proptest(filter = "|&v| v != Some(0)")]
    pub optional_sfixed64: Option<i64>,
    #[proptest(filter = "|&v| v != Some(0.0)")]
    pub optional_float: Option<f32>,
    #[proptest(filter = "|&v| v != Some(0.0)")]
    pub optional_double: Option<f64>,
    #[proptest(filter = "|&v| v != Some(false)")]
    pub optional_bool: Option<bool>,
    #[proptest(filter = "|v| v.as_deref() != Some(\"\")")]
    pub optional_string: Option<String>,
    #[proptest(filter = "|v| v.as_deref() != Some(&[])")]
    pub optional_bytes: Option<Vec<u8>>,

    pub repeated_int32: Vec<i32>,
    pub repeated_int64: Vec<i64>,
    pub repeated_uint32: Vec<u32>,
    pub repeated_uint64: Vec<u64>,
    pub repeated_sint32: Vec<i32>,
    pub repeated_sint64: Vec<i64>,
    pub repeated_fixed32: Vec<u32>,
    pub repeated_fixed64: Vec<u64>,
    pub repeated_sfixed32: Vec<i32>,
    pub repeated_sfixed64: Vec<i64>,
    pub repeated_float: Vec<f32>,
    pub repeated_double: Vec<f64>,
    pub repeated_bool: Vec<bool>,
    pub repeated_string: Vec<String>,
    pub repeated_bytes: Vec<Vec<u8>>,
    pub packed_int32: Vec<i32>,
    pub packed_int64: Vec<i64>,
    pub packed_uint32: Vec<u32>,
    pub packed_uint64: Vec<u64>,
    pub packed_sint32: Vec<i32>,
    pub packed_sint64: Vec<i64>,
    pub packed_fixed32: Vec<u32>,
    pub packed_fixed64: Vec<u64>,
    pub packed_sfixed32: Vec<i32>,
    pub packed_sfixed64: Vec<i64>,
    pub packed_float: Vec<f32>,
    pub packed_double: Vec<f64>,
    pub packed_bool: Vec<bool>,
    pub packed_string: Vec<String>,
    pub packed_bytes: Vec<Vec<u8>>,
}

#[test]
fn scalar_types_varints() {
    use femtopb::Message as _;
    let expected = ScalarTypes {
        int32: 1,
        int64: 2,
        uint32: 3,
        uint64: 4,
        sint32: -5,
        sint64: -6,
        _bool: true,
        required_int32: 15,
        required_int64: 16,
        required_uint32: 17,
        required_uint64: 18,
        required_sint32: -19,
        required_sint64: -20,
        required_bool: true,
        optional_int32: Some(29),
        optional_int64: Some(30),
        optional_uint32: Some(31),
        optional_uint64: Some(32),
        optional_sint32: Some(-33),
        optional_sint64: Some(-34),
        optional_bool: Some(true),
        ..Default::default()
    };
    let mut buf = vec![0u8; expected.encoded_len()];
    expected.encode(&mut buf.as_mut_slice()).unwrap();
    let result = ScalarTypes::decode(&buf);
    assert_eq!(Ok(expected), result);
}

#[test]
fn scalar_types_fixed_sizes() {
    use femtopb::Message as _;
    let expected = ScalarTypes {
        fixed32: 7,
        fixed64: 8,
        sfixed32: -9,
        sfixed64: -10,
        float: 11.0,
        double: 12.0,
        required_fixed32: 21,
        required_fixed64: 22,
        required_sfixed32: -23,
        required_sfixed64: -24,
        required_float: 25.0,
        required_double: 26.0,
        optional_fixed32: Some(35),
        optional_fixed64: Some(36),
        optional_sfixed32: Some(-37),
        optional_sfixed64: Some(-38),
        optional_float: Some(39.0),
        optional_double: Some(40.0),
        ..Default::default()
    };
    let mut buf = vec![0u8; expected.encoded_len()];
    expected.encode(&mut buf.as_mut_slice()).unwrap();
    let result = ScalarTypes::decode(&buf);
    assert_eq!(Ok(expected), result);
}

#[test]
fn scalar_types_length_delimited() {
    use femtopb::Message as _;
    let expected = ScalarTypes {
        string: "thirteen",
        bytes: b"\x00fourteen\x01",
        required_string: "twenty-seven",
        required_bytes: b"\x02twenty-eight\x03",
        optional_string: Some("forty-one"),
        optional_bytes: Some(b"\x04forty-two\x05"),
        ..Default::default()
    };
    let mut buf = vec![0u8; expected.encoded_len()];
    expected.encode(&mut buf.as_mut_slice()).unwrap();
    let result = ScalarTypes::decode(&buf);
    assert_eq!(Ok(expected), result);
}

#[test]
fn scalar_types_repeated() {
    use femtopb::Message as _;
    let expected = ScalarTypes {
        repeated_int32: repeated::Repeated::from_slice(&[45i32, 46]),
        repeated_int64: repeated::Repeated::from_slice(&[47i64, 48]),
        repeated_uint32: repeated::Repeated::from_slice(&[49u32, 50]),
        repeated_uint64: repeated::Repeated::from_slice(&[51u64, 52]),
        repeated_sint32: repeated::Repeated::from_slice(&[-53, 54]),
        repeated_sint64: repeated::Repeated::from_slice(&[-55, 56]),
        repeated_fixed32: repeated::Repeated::from_slice(&[57, 58]),
        repeated_fixed64: repeated::Repeated::from_slice(&[59, 60]),
        repeated_sfixed32: repeated::Repeated::from_slice(&[-61, 62]),
        repeated_sfixed64: repeated::Repeated::from_slice(&[-63, 64]),
        repeated_float: repeated::Repeated::from_slice(&[-65.0, 66.0]),
        repeated_double: repeated::Repeated::from_slice(&[-67.0, 68.0]),
        repeated_bool: repeated::Repeated::from_slice(&[true, false]),
        repeated_string: repeated::Repeated::from_slice(&["a", "b", "c"]),
        repeated_bytes: repeated::Repeated::from_slice(&[b"d", b"e", b"f", b"\x05"]),
        ..Default::default()
    };
    let mut buf = vec![0u8; expected.encoded_len()];
    expected.encode(&mut buf.as_mut_slice()).unwrap();
    let result = ScalarTypes::decode(&buf);
    assert_eq!(Ok(expected), result);
}

#[test]
fn scalar_types_packed() {
    use femtopb::Message as _;
    let expected = ScalarTypes {
        packed_int32: packed::Packed::from_slice(&[69, 70]),
        packed_int64: packed::Packed::from_slice(&[71, 72]),
        packed_uint32: packed::Packed::from_slice(&[73, 74]),
        packed_uint64: packed::Packed::from_slice(&[75, 76]),
        packed_sint32: packed::Packed::from_slice(&[-77, 78]),
        packed_sint64: packed::Packed::from_slice(&[-79, 80]),
        packed_fixed32: packed::Packed::from_slice(&[81, 82]),
        packed_fixed64: packed::Packed::from_slice(&[83, 84]),
        packed_sfixed32: packed::Packed::from_slice(&[-85, 86]),
        packed_sfixed64: packed::Packed::from_slice(&[-87, 88]),
        packed_float: packed::Packed::from_slice(&[-89.0, 90.0]),
        packed_double: packed::Packed::from_slice(&[-91.0, 92.0]),
        packed_bool: packed::Packed::from_slice(&[true, false]),
        ..Default::default()
    };
    let mut buf = vec![0u8; expected.encoded_len()];
    expected.encode(&mut buf.as_mut_slice()).unwrap();
    let result = ScalarTypes::decode(&buf);
    assert_eq!(Ok(expected), result);
}

#[test]
fn scalar_types_kitchen_sink() {
    use femtopb::Message as _;
    // Typing this out manually to avoid bugs in any sort of automation like macros, etc...
    let expected = ScalarTypes {
        int32: 1,
        int64: 2,
        uint32: 3,
        uint64: 4,
        sint32: -5,
        sint64: -6,
        fixed32: 7,
        fixed64: 8,
        sfixed32: -9,
        sfixed64: -10,
        float: 11.0,
        double: 12.0,
        _bool: true,
        string: "thirteen",
        bytes: b"\x00fourteen\x01",
        required_int32: 15,
        required_int64: 16,
        required_uint32: 17,
        required_uint64: 18,
        required_sint32: -19,
        required_sint64: -20,
        required_fixed32: 21,
        required_fixed64: 22,
        required_sfixed32: -23,
        required_sfixed64: -24,
        required_float: 25.0,
        required_double: 26.0,
        required_bool: true,
        required_string: "twenty-seven",
        required_bytes: b"\x02twenty-eight\x03",
        optional_int32: Some(29),
        optional_int64: Some(30),
        optional_uint32: Some(31),
        optional_uint64: Some(32),
        optional_sint32: Some(-33),
        optional_sint64: Some(-34),
        optional_fixed32: Some(35),
        optional_fixed64: Some(36),
        optional_sfixed32: Some(-37),
        optional_sfixed64: Some(-38),
        optional_float: Some(39.0),
        optional_double: Some(40.0),
        optional_bool: Some(true),
        optional_string: Some("forty-one"),
        optional_bytes: Some(b"\x04forty-two\x05"),
        repeated_int32: repeated::Repeated::from_slice(&[45i32, 46]),
        repeated_int64: repeated::Repeated::from_slice(&[47i64, 48]),
        repeated_uint32: repeated::Repeated::from_slice(&[49u32, 50]),
        repeated_uint64: repeated::Repeated::from_slice(&[51u64, 52]),
        repeated_sint32: repeated::Repeated::from_slice(&[-53, 54]),
        repeated_sint64: repeated::Repeated::from_slice(&[-55, 56]),
        repeated_fixed32: repeated::Repeated::from_slice(&[57, 58]),
        repeated_fixed64: repeated::Repeated::from_slice(&[59, 60]),
        repeated_sfixed32: repeated::Repeated::from_slice(&[-61, 62]),
        repeated_sfixed64: repeated::Repeated::from_slice(&[-63, 64]),
        repeated_float: repeated::Repeated::from_slice(&[-65.0, 66.0]),
        repeated_double: repeated::Repeated::from_slice(&[-67.0, 68.0]),
        repeated_bool: repeated::Repeated::from_slice(&[true, false]),
        repeated_string: repeated::Repeated::from_slice(&["a", "b", "c"]),
        repeated_bytes: repeated::Repeated::from_slice(&[b"d", b"e", b"f", b"\x05"]),
        packed_int32: packed::Packed::from_slice(&[69, 70]),
        packed_int64: packed::Packed::from_slice(&[71, 72]),
        packed_uint32: packed::Packed::from_slice(&[73, 74]),
        packed_uint64: packed::Packed::from_slice(&[75, 76]),
        packed_sint32: packed::Packed::from_slice(&[-77, 78]),
        packed_sint64: packed::Packed::from_slice(&[-79, 80]),
        packed_fixed32: packed::Packed::from_slice(&[81, 82]),
        packed_fixed64: packed::Packed::from_slice(&[83, 84]),
        packed_sfixed32: packed::Packed::from_slice(&[-85, 86]),
        packed_sfixed64: packed::Packed::from_slice(&[-87, 88]),
        packed_float: packed::Packed::from_slice(&[-89.0, 90.0]),
        packed_double: packed::Packed::from_slice(&[-91.0, 92.0]),
        packed_bool: packed::Packed::from_slice(&[true, false]),
        packed_string: repeated::Repeated::from_slice(&["g", "h", "i"]),
        packed_bytes: repeated::Repeated::from_slice(&[b"j", b"k", b"l", b"\x06"]),
        unknown_fields: femtopb::UnknownFields::default(),
    };
    let mut buf = vec![0u8; expected.encoded_len()];
    expected.encode(&mut buf.as_mut_slice()).unwrap();
    let result = ScalarTypes::decode(&buf);
    assert_eq!(Ok(expected), result);
}

proptest::proptest! {
    #[test]
    fn roundtrip_scalar_types(arbitrary: ArbitraryScalarTypes) {
        use femtopb::Message as _;
        // Convert these into vecs-of-slices instead of vecs-of-owned values.
        let repeated_string = arbitrary.repeated_string.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        let repeated_bytes = arbitrary.repeated_bytes.iter().map(|s|s.as_slice()).collect::<Vec<_>>();
        let packed_string = arbitrary.packed_string.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        let packed_bytes = arbitrary.packed_bytes.iter().map(|s|s.as_slice()).collect::<Vec<_>>();

        // Typing this out manually to avoid bugs in any sort of automation like macros, etc...
        let expected = ScalarTypes {
            int32: arbitrary.int32,
            int64: arbitrary.int64,
            uint32: arbitrary.uint32,
            uint64: arbitrary.uint64,
            sint32: arbitrary.sint32,
            sint64: arbitrary.sint64,
            fixed32: arbitrary.fixed32,
            fixed64: arbitrary.fixed64,
            sfixed32: arbitrary.sfixed32,
            sfixed64: arbitrary.sfixed64,
            float: arbitrary.float,
            double: arbitrary.double,
            _bool: arbitrary._bool,
            string: arbitrary.string.as_ref(),
            bytes: arbitrary.bytes.as_ref(),
            required_int32: arbitrary.required_int32,
            required_int64: arbitrary.required_int64,
            required_uint32: arbitrary.required_uint32,
            required_uint64: arbitrary.required_uint64,
            required_sint32: arbitrary.required_sint32,
            required_sint64: arbitrary.required_sint64,
            required_fixed32: arbitrary.required_fixed32,
            required_fixed64: arbitrary.required_fixed64,
            required_sfixed32: arbitrary.required_sfixed32,
            required_sfixed64: arbitrary.required_sfixed64,
            required_float: arbitrary.required_float,
            required_double: arbitrary.required_double,
            required_bool: arbitrary.required_bool,
            required_string: arbitrary.required_string.as_ref(),
            required_bytes: arbitrary.required_bytes.as_ref(),
            optional_int32: arbitrary.optional_int32,
            optional_int64: arbitrary.optional_int64,
            optional_uint32: arbitrary.optional_uint32,
            optional_uint64: arbitrary.optional_uint64,
            optional_sint32: arbitrary.optional_sint32,
            optional_sint64: arbitrary.optional_sint64,
            optional_fixed32: arbitrary.optional_fixed32,
            optional_fixed64: arbitrary.optional_fixed64,
            optional_sfixed32: arbitrary.optional_sfixed32,
            optional_sfixed64: arbitrary.optional_sfixed64,
            optional_float: arbitrary.optional_float,
            optional_double: arbitrary.optional_double,
            optional_bool: arbitrary.optional_bool,
            optional_string: arbitrary.optional_string.as_deref(),
            optional_bytes: arbitrary.optional_bytes.as_deref(),
            repeated_int32: arbitrary.repeated_int32.as_slice().into(),
            repeated_int64: arbitrary.repeated_int64.as_slice().into(),
            repeated_uint32: arbitrary.repeated_uint32.as_slice().into(),
            repeated_uint64: arbitrary.repeated_uint64.as_slice().into(),
            repeated_sint32: arbitrary.repeated_sint32.as_slice().into(),
            repeated_sint64: arbitrary.repeated_sint64.as_slice().into(),
            repeated_fixed32: arbitrary.repeated_fixed32.as_slice().into(),
            repeated_fixed64: arbitrary.repeated_fixed64.as_slice().into(),
            repeated_sfixed32: arbitrary.repeated_sfixed32.as_slice().into(),
            repeated_sfixed64: arbitrary.repeated_sfixed64.as_slice().into(),
            repeated_float: arbitrary.repeated_float.as_slice().into(),
            repeated_double: arbitrary.repeated_double.as_slice().into(),
            repeated_bool: arbitrary.repeated_bool.as_slice().into(),
            repeated_string: repeated_string.as_slice().into(),
            repeated_bytes: repeated_bytes.as_slice().into(),
            packed_int32: arbitrary.packed_int32.as_slice().into(),
            packed_int64: arbitrary.packed_int64.as_slice().into(),
            packed_uint32: arbitrary.packed_uint32.as_slice().into(),
            packed_uint64: arbitrary.packed_uint64.as_slice().into(),
            packed_sint32: arbitrary.packed_sint32.as_slice().into(),
            packed_sint64: arbitrary.packed_sint64.as_slice().into(),
            packed_fixed32: arbitrary.packed_fixed32.as_slice().into(),
            packed_fixed64: arbitrary.packed_fixed64.as_slice().into(),
            packed_sfixed32: arbitrary.packed_sfixed32.as_slice().into(),
            packed_sfixed64: arbitrary.packed_sfixed64.as_slice().into(),
            packed_float: arbitrary.packed_float.as_slice().into(),
            packed_double: arbitrary.packed_double.as_slice().into(),
            packed_bool: arbitrary.packed_bool.as_slice().into(),
            packed_string: packed_string.as_slice().into(),
            packed_bytes: packed_bytes.as_slice().into(),
            unknown_fields: femtopb::UnknownFields::default(),
        };
        let mut buf = vec![0u8; expected.encoded_len()];
        expected.encode(&mut buf.as_mut_slice()).unwrap();
        let result = ScalarTypes::decode(&buf);
        proptest::prop_assert_eq!(Ok(expected), result);
    }

    #[test]
    fn roundtrip_prost_scalar_types(arbitrary: ArbitraryScalarTypes) {
        use femtopb::Message as _;
        // Typing this out manually to avoid bugs in any sort of automation like macros, etc...
        let prost_expected = ProstScalarTypes {
            int32: arbitrary.int32,
            int64: arbitrary.int64,
            uint32: arbitrary.uint32,
            uint64: arbitrary.uint64,
            sint32: arbitrary.sint32,
            sint64: arbitrary.sint64,
            fixed32: arbitrary.fixed32,
            fixed64: arbitrary.fixed64,
            sfixed32: arbitrary.sfixed32,
            sfixed64: arbitrary.sfixed64,
            float: arbitrary.float,
            double: arbitrary.double,
            _bool: arbitrary._bool,
            string: arbitrary.string.clone(),
            bytes: arbitrary.bytes.clone().into(),
            required_int32: arbitrary.required_int32,
            required_int64: arbitrary.required_int64,
            required_uint32: arbitrary.required_uint32,
            required_uint64: arbitrary.required_uint64,
            required_sint32: arbitrary.required_sint32,
            required_sint64: arbitrary.required_sint64,
            required_fixed32: arbitrary.required_fixed32,
            required_fixed64: arbitrary.required_fixed64,
            required_sfixed32: arbitrary.required_sfixed32,
            required_sfixed64: arbitrary.required_sfixed64,
            required_float: arbitrary.required_float,
            required_double: arbitrary.required_double,
            required_bool: arbitrary.required_bool,
            required_string: arbitrary.required_string.clone(),
            required_bytes: arbitrary.required_bytes.clone().into(),
            optional_int32: arbitrary.optional_int32,
            optional_int64: arbitrary.optional_int64,
            optional_uint32: arbitrary.optional_uint32,
            optional_uint64: arbitrary.optional_uint64,
            optional_sint32: arbitrary.optional_sint32,
            optional_sint64: arbitrary.optional_sint64,
            optional_fixed32: arbitrary.optional_fixed32,
            optional_fixed64: arbitrary.optional_fixed64,
            optional_sfixed32: arbitrary.optional_sfixed32,
            optional_sfixed64: arbitrary.optional_sfixed64,
            optional_float: arbitrary.optional_float,
            optional_double: arbitrary.optional_double,
            optional_bool: arbitrary.optional_bool,
            optional_string: arbitrary.optional_string.clone(),
            optional_bytes: arbitrary.optional_bytes.as_ref().map(|b| b.clone().into()).clone(),
            repeated_int32: arbitrary.repeated_int32.clone(),
            repeated_int64: arbitrary.repeated_int64.clone(),
            repeated_uint32: arbitrary.repeated_uint32.clone(),
            repeated_uint64: arbitrary.repeated_uint64.clone(),
            repeated_sint32: arbitrary.repeated_sint32.clone(),
            repeated_sint64: arbitrary.repeated_sint64.clone(),
            repeated_fixed32: arbitrary.repeated_fixed32.clone(),
            repeated_fixed64: arbitrary.repeated_fixed64.clone(),
            repeated_sfixed32: arbitrary.repeated_sfixed32.clone(),
            repeated_sfixed64: arbitrary.repeated_sfixed64.clone(),
            repeated_float: arbitrary.repeated_float.clone(),
            repeated_double: arbitrary.repeated_double.clone(),
            repeated_bool: arbitrary.repeated_bool.clone(),
            repeated_string: arbitrary.repeated_string.clone(),
            repeated_bytes: arbitrary.repeated_bytes.iter().map(|b| b.clone().into()).collect::<Vec<_>>(),
            packed_int32: arbitrary.packed_int32.clone(),
            packed_int64: arbitrary.packed_int64.clone(),
            packed_uint32: arbitrary.packed_uint32.clone(),
            packed_uint64: arbitrary.packed_uint64.clone(),
            packed_sint32: arbitrary.packed_sint32.clone(),
            packed_sint64: arbitrary.packed_sint64.clone(),
            packed_fixed32: arbitrary.packed_fixed32.clone(),
            packed_fixed64: arbitrary.packed_fixed64.clone(),
            packed_sfixed32: arbitrary.packed_sfixed32.clone(),
            packed_sfixed64: arbitrary.packed_sfixed64.clone(),
            packed_float: arbitrary.packed_float.clone(),
            packed_double: arbitrary.packed_double.clone(),
            packed_bool: arbitrary.packed_bool.clone(),
            packed_string: arbitrary.packed_string.clone(),
            packed_bytes: arbitrary.packed_bytes.iter().map(|b| b.clone().into()).collect::<Vec<_>>(),
        };

        // Convert these into vecs-of-slices instead of vecs-of-owned values.
        let repeated_string = arbitrary.repeated_string.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        let repeated_bytes = arbitrary.repeated_bytes.iter().map(|s|s.as_slice()).collect::<Vec<_>>();
        let packed_string = arbitrary.packed_string.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        let packed_bytes = arbitrary.packed_bytes.iter().map(|s|s.as_slice()).collect::<Vec<_>>();

        // Typing this out manually to avoid bugs in any sort of automation like macros, etc...
        let expected = ScalarTypes {
            int32: arbitrary.int32,
            int64: arbitrary.int64,
            uint32: arbitrary.uint32,
            uint64: arbitrary.uint64,
            sint32: arbitrary.sint32,
            sint64: arbitrary.sint64,
            fixed32: arbitrary.fixed32,
            fixed64: arbitrary.fixed64,
            sfixed32: arbitrary.sfixed32,
            sfixed64: arbitrary.sfixed64,
            float: arbitrary.float,
            double: arbitrary.double,
            _bool: arbitrary._bool,
            string: arbitrary.string.as_ref(),
            bytes: arbitrary.bytes.as_ref(),
            required_int32: arbitrary.required_int32,
            required_int64: arbitrary.required_int64,
            required_uint32: arbitrary.required_uint32,
            required_uint64: arbitrary.required_uint64,
            required_sint32: arbitrary.required_sint32,
            required_sint64: arbitrary.required_sint64,
            required_fixed32: arbitrary.required_fixed32,
            required_fixed64: arbitrary.required_fixed64,
            required_sfixed32: arbitrary.required_sfixed32,
            required_sfixed64: arbitrary.required_sfixed64,
            required_float: arbitrary.required_float,
            required_double: arbitrary.required_double,
            required_bool: arbitrary.required_bool,
            required_string: arbitrary.required_string.as_ref(),
            required_bytes: arbitrary.required_bytes.as_ref(),
            optional_int32: arbitrary.optional_int32,
            optional_int64: arbitrary.optional_int64,
            optional_uint32: arbitrary.optional_uint32,
            optional_uint64: arbitrary.optional_uint64,
            optional_sint32: arbitrary.optional_sint32,
            optional_sint64: arbitrary.optional_sint64,
            optional_fixed32: arbitrary.optional_fixed32,
            optional_fixed64: arbitrary.optional_fixed64,
            optional_sfixed32: arbitrary.optional_sfixed32,
            optional_sfixed64: arbitrary.optional_sfixed64,
            optional_float: arbitrary.optional_float,
            optional_double: arbitrary.optional_double,
            optional_bool: arbitrary.optional_bool,
            optional_string: arbitrary.optional_string.as_deref(),
            optional_bytes: arbitrary.optional_bytes.as_deref(),
            repeated_int32: arbitrary.repeated_int32.as_slice().into(),
            repeated_int64: arbitrary.repeated_int64.as_slice().into(),
            repeated_uint32: arbitrary.repeated_uint32.as_slice().into(),
            repeated_uint64: arbitrary.repeated_uint64.as_slice().into(),
            repeated_sint32: arbitrary.repeated_sint32.as_slice().into(),
            repeated_sint64: arbitrary.repeated_sint64.as_slice().into(),
            repeated_fixed32: arbitrary.repeated_fixed32.as_slice().into(),
            repeated_fixed64: arbitrary.repeated_fixed64.as_slice().into(),
            repeated_sfixed32: arbitrary.repeated_sfixed32.as_slice().into(),
            repeated_sfixed64: arbitrary.repeated_sfixed64.as_slice().into(),
            repeated_float: arbitrary.repeated_float.as_slice().into(),
            repeated_double: arbitrary.repeated_double.as_slice().into(),
            repeated_bool: arbitrary.repeated_bool.as_slice().into(),
            repeated_string: repeated_string.as_slice().into(),
            repeated_bytes: repeated_bytes.as_slice().into(),
            packed_int32: arbitrary.packed_int32.as_slice().into(),
            packed_int64: arbitrary.packed_int64.as_slice().into(),
            packed_uint32: arbitrary.packed_uint32.as_slice().into(),
            packed_uint64: arbitrary.packed_uint64.as_slice().into(),
            packed_sint32: arbitrary.packed_sint32.as_slice().into(),
            packed_sint64: arbitrary.packed_sint64.as_slice().into(),
            packed_fixed32: arbitrary.packed_fixed32.as_slice().into(),
            packed_fixed64: arbitrary.packed_fixed64.as_slice().into(),
            packed_sfixed32: arbitrary.packed_sfixed32.as_slice().into(),
            packed_sfixed64: arbitrary.packed_sfixed64.as_slice().into(),
            packed_float: arbitrary.packed_float.as_slice().into(),
            packed_double: arbitrary.packed_double.as_slice().into(),
            packed_bool: arbitrary.packed_bool.as_slice().into(),
            packed_string: packed_string.as_slice().into(),
            packed_bytes: packed_bytes.as_slice().into(),
            unknown_fields: femtopb::UnknownFields::default(),
        };
        let mut buf = vec![];
        prost::Message::encode(&prost_expected, &mut buf).unwrap();
        let result = ScalarTypes::decode(&buf);
        proptest::prop_assert_eq!(Ok(expected.clone()), result);

        buf.clear();
        buf.resize(expected.encoded_len(), 0);
        expected.encode(&mut buf.as_mut_slice()).unwrap();
        let result = prost::Message::decode(bytes::Bytes::from(buf));
        proptest::prop_assert_eq!(Ok(prost_expected), result);
    }
}
