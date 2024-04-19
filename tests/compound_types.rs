#[derive(
    Clone, Copy, Debug, Default, PartialEq, femtopb::Enumeration, proptest_derive::Arbitrary,
)]
pub enum BasicEnumeration {
    #[default]
    ZERO = 0,
    ONE = 1,
    TWO = 2,
    THREE = 3,
}

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Basic<'a> {
    #[femtopb(int32, tag = 1)]
    pub int32: i32,

    #[femtopb(bool, repeated, tag = 2)]
    pub bools: femtopb::Repeated<'a, bool, femtopb::item_encoding::Bool>,

    #[femtopb(string, tag = 3)]
    pub string: &'a str,

    #[femtopb(string, optional, tag = 4)]
    pub optional_string: Option<&'a str>,

    #[femtopb(enumeration, tag = 5)]
    pub enumeration: femtopb::EnumValue<BasicEnumeration>,

    #[femtopb(oneof, tags = [8, 9])]
    pub oneof: Option<BasicOneof<'a>>,

    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct Compound<'a> {
    #[femtopb(message, optional, tag = 1)]
    pub optional_message: Option<Basic<'a>>,

    #[femtopb(message, required, tag = 2)]
    pub required_message: Basic<'a>,

    #[femtopb(message, repeated, tag = 3)]
    pub repeated_message:
        femtopb::Repeated<'a, Basic<'a>, femtopb::item_encoding::Message<'a, Basic<'a>>>,

    #[femtopb(enumeration, packed, tag = 4)]
    pub packed_enumeration: femtopb::Packed<
        'a,
        femtopb::EnumValue<BasicEnumeration>,
        femtopb::item_encoding::Enum<BasicEnumeration>,
    >,

    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct PackedEnum<'a> {
    #[femtopb(enumeration, packed, tag = 1)]
    pub packed_enumeration: femtopb::Packed<
        'a,
        femtopb::EnumValue<BasicEnumeration>,
        femtopb::item_encoding::Enum<BasicEnumeration>,
    >,

    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

#[derive(Clone, Debug, PartialEq, femtopb::Oneof)]
pub enum BasicOneof<'a> {
    #[femtopb(int32, tag = 8)]
    Int(i32),
    #[femtopb(string, tag = 9)]
    String(&'a str),
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, femtopb::Message)]
pub struct DefaultValues<'a> {
    #[femtopb(int32, tag = 1, default = 42)]
    pub int32: i32,

    #[femtopb(int32, optional, tag = 2, default = 88)]
    pub optional_int32: Option<i32>,

    #[femtopb(string, tag = 3, default = "forty two")]
    pub string: &'a str,

    #[femtopb(enumeration, tag = 4, default = BasicEnumeration::ONE)]
    pub enumeration: femtopb::EnumValue<BasicEnumeration>,

    #[femtopb(enumeration, optional, tag = 5, default = BasicEnumeration::TWO)]
    pub optional_enumeration: Option<femtopb::EnumValue<BasicEnumeration>>,

    #[femtopb(enumeration, repeated, tag = 6)]
    pub repeated_enumeration: femtopb::Repeated<
        'a,
        femtopb::EnumValue<BasicEnumeration>,
        femtopb::item_encoding::Enum<BasicEnumeration>,
    >,

    #[femtopb(bytes, tag = 7, default = b"\x00\x01\x02")]
    pub bytes: &'a [u8],

    #[femtopb(float, tag = 8, default = 10.0)]
    pub float: f32,

    #[femtopb(double, optional, tag = 9, default = 42.0)]
    pub double: Option<f64>,

    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}

/// A protobuf enum.
#[allow(clippy::upper_case_acronyms, clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, Debug, PartialEq, prost::Enumeration)]
pub enum ProstBasicEnumeration {
    ZERO = 0,
    ONE = 1,
    TWO = 2,
    THREE = 3,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProstBasic {
    #[prost(int32, tag = "1")]
    pub int32: i32,

    #[prost(bool, repeated, packed = "false", tag = "2")]
    pub bools: Vec<bool>,

    #[prost(string, tag = "3")]
    pub string: String,

    #[prost(string, optional, tag = "4")]
    pub optional_string: Option<String>,

    #[prost(enumeration = "ProstBasicEnumeration", tag = "5")]
    pub enumeration: i32,

    #[prost(oneof = "ProstBasicOneof", tags = "8, 9")]
    pub oneof: Option<ProstBasicOneof>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, prost::Message)]
pub struct ProstCompound {
    #[prost(message, optional, tag = "1")]
    pub optional_message: Option<ProstBasic>,

    #[prost(message, required, tag = "2")]
    pub required_message: ProstBasic,

    #[prost(message, repeated, tag = "3")]
    pub repeated_message: Vec<ProstBasic>,

    #[prost(map = "sint32, message", tag = "4")]
    pub message_map: ::std::collections::HashMap<i32, ProstBasic>,

    #[prost(btree_map = "sint32, message", tag = "5")]
    pub message_btree_map: prost::alloc::collections::BTreeMap<i32, ProstBasic>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, prost::Oneof)]
pub enum ProstBasicOneof {
    #[prost(int32, tag = "8")]
    Int(i32),
    #[prost(string, tag = "9")]
    String(String),
}

#[derive(Clone, Debug, PartialEq, proptest_derive::Arbitrary)]
pub struct ArbitraryBasic {
    pub int32: i32,
    pub bools: Vec<bool>,
    pub string: String,
    #[proptest(filter = "|v| v.as_deref() != Some(\"\")")]
    pub optional_string: Option<String>,
    pub enumeration: ArbitraryBasicEnumerationEnumValue,
    pub oneof: Option<ArbitraryBasicOneof>,
}

#[derive(Clone, Copy, Debug, PartialEq, proptest_derive::Arbitrary)]
pub enum ArbitraryBasicEnumerationEnumValue {
    Known(BasicEnumeration),
    Unknown(i32),
}

#[derive(Clone, Debug, PartialEq, proptest_derive::Arbitrary)]
#[proptest(
    filter = "|v| v != &ArbitraryBasicOneof::Int(0) && v != &ArbitraryBasicOneof::String(\"\".to_owned())"
)]
pub enum ArbitraryBasicOneof {
    Int(i32),
    String(String),
}

#[derive(Clone, Debug, PartialEq, proptest_derive::Arbitrary)]
pub struct ArbitraryCompound {
    pub optional_message: Option<ArbitraryBasic>,
    pub required_message: ArbitraryBasic,
    pub repeated_message: Vec<ArbitraryBasic>,
    pub packed_enumeration: Vec<ArbitraryBasicEnumerationEnumValue>,
}

impl<'a> Basic<'a> {
    fn from_arbitrary(arbitrary: &'a ArbitraryBasic) -> Self {
        let enumeration = match arbitrary.enumeration {
            ArbitraryBasicEnumerationEnumValue::Known(v) => femtopb::EnumValue::Known(v),
            ArbitraryBasicEnumerationEnumValue::Unknown(v) => femtopb::EnumValue::Unknown(v),
        };
        let oneof = match arbitrary.oneof {
            None => None,
            Some(ArbitraryBasicOneof::Int(i)) => Some(BasicOneof::Int(i)),
            Some(ArbitraryBasicOneof::String(ref s)) => Some(BasicOneof::String(s.as_str())),
        };
        Self {
            int32: arbitrary.int32,
            bools: arbitrary.bools.as_slice().into(),
            string: arbitrary.string.as_str(),
            optional_string: arbitrary.optional_string.as_deref(),
            enumeration,
            oneof,
            unknown_fields: Default::default(),
        }
    }
}

impl ProstBasic {
    fn from_arbitrary(arbitrary: &ArbitraryBasic) -> Self {
        let enumeration = arbitrary.enumeration.to_prost();
        let oneof = match arbitrary.oneof {
            None => None,
            Some(ArbitraryBasicOneof::Int(i)) => Some(ProstBasicOneof::Int(i)),
            Some(ArbitraryBasicOneof::String(ref s)) => Some(ProstBasicOneof::String(s.clone())),
        };
        Self {
            int32: arbitrary.int32,
            bools: arbitrary.bools.as_slice().into(),
            string: arbitrary.string.clone(),
            optional_string: arbitrary.optional_string.clone(),
            enumeration,
            oneof,
        }
    }
}

impl ArbitraryBasicEnumerationEnumValue {
    fn to_femtopb(self) -> femtopb::EnumValue<BasicEnumeration> {
        match self {
            ArbitraryBasicEnumerationEnumValue::Known(v) => femtopb::EnumValue::Known(v),
            ArbitraryBasicEnumerationEnumValue::Unknown(i) => femtopb::EnumValue::Unknown(i),
        }
    }
    fn to_prost(self) -> i32 {
        match self {
            ArbitraryBasicEnumerationEnumValue::Known(v) => femtopb::EnumValue::Known(v).to_raw(),
            ArbitraryBasicEnumerationEnumValue::Unknown(i) => i,
        }
    }
}

proptest::proptest! {
    #[test]
    fn roundtrip_basic_types(arbitrary: ArbitraryBasic) {
        use femtopb::Message as _;

        let expected = Basic::from_arbitrary(&arbitrary);

        let mut buf = vec![0u8; expected.encoded_len()];
        expected.encode(&mut buf.as_mut_slice()).unwrap();
        let result = Basic::decode(&buf);
        proptest::prop_assert_eq!(Ok(expected), result);
    }

    #[test]
    fn roundtrip_prost_basic_types(arbitrary: ArbitraryBasic) {
        use femtopb::Message as _;

        let expected = Basic::from_arbitrary(&arbitrary);
        let prost_expected = ProstBasic::from_arbitrary(&arbitrary);

        let mut buf = vec![];
        prost::Message::encode(&prost_expected, &mut buf).unwrap();
        let result = Basic::decode(&buf);
        proptest::prop_assert_eq!(Ok(expected.clone()), result);

        buf.clear();
        buf.resize(expected.encoded_len(), 0);
        expected.encode(&mut buf.as_mut_slice()).unwrap();
        let result = prost::Message::decode(bytes::Bytes::from(buf));
        proptest::prop_assert_eq!(Ok(prost_expected), result);
    }

    #[test]
    fn roundtrip_compound_types(arbitrary: ArbitraryCompound) {
        use femtopb::Message as _;

        let repeated_message = arbitrary
            .repeated_message
            .iter()
            .map(|a| Basic::from_arbitrary(a))
            .collect::<Vec<_>>();
        let packed_enumeration = arbitrary
            .packed_enumeration
            .into_iter()
            .map(|e| e.to_femtopb())
            .collect::<Vec<_>>();
        let expected = Compound {
            optional_message: arbitrary
                .optional_message
                .as_ref()
                .map(Basic::from_arbitrary),
            required_message: Basic::from_arbitrary(&arbitrary.required_message),
            repeated_message: femtopb::repeated::Repeated::from_slice(repeated_message.as_slice()),
            packed_enumeration: femtopb::Packed::from_slice(packed_enumeration.as_slice()),
            unknown_fields: Default::default(),
        };

        let mut buf = vec![0u8; expected.encoded_len()];
        expected.encode(&mut buf.as_mut_slice()).unwrap();
        let result = Compound::decode(&buf);
        proptest::prop_assert_eq!(Ok(expected), result);
    }
}

#[test]
fn packed_enums() {
    use femtopb::Message as _;

    let expected = PackedEnum {
        packed_enumeration: femtopb::Packed::from_slice(&[femtopb::EnumValue::Known(
            BasicEnumeration::ZERO,
        )]),
        ..Default::default()
    };

    let mut buf = vec![0u8; expected.encoded_len()];
    expected.encode(&mut buf.as_mut_slice()).unwrap();
    let result = PackedEnum::decode(&buf);
    assert_eq!(Ok(expected), result);
}
