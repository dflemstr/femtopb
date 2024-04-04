#[derive(defmt::Format)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct PublicImportMessage<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub e: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(defmt::Format)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ImportMessage<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub d: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(defmt::Format)]
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum ImportEnum {
    #[default]
    ImportFoo = 7,
    ImportBar = 8,
    ImportBaz = 9,
}
impl ImportEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ImportEnum::ImportFoo => "IMPORT_FOO",
            ImportEnum::ImportBar => "IMPORT_BAR",
            ImportEnum::ImportBaz => "IMPORT_BAZ",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IMPORT_FOO" => Some(Self::ImportFoo),
            "IMPORT_BAR" => Some(Self::ImportBar),
            "IMPORT_BAZ" => Some(Self::ImportBaz),
            _ => None,
        }
    }
}
/// To use an enum in a map, it must has the first value as 0.
#[derive(defmt::Format)]
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum ImportEnumForMap {
    #[default]
    Unknown = 0,
    Foo = 1,
    Bar = 2,
}
impl ImportEnumForMap {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ImportEnumForMap::Unknown => "UNKNOWN",
            ImportEnumForMap::Foo => "FOO",
            ImportEnumForMap::Bar => "BAR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "FOO" => Some(Self::Foo),
            "BAR" => Some(Self::Bar),
            _ => None,
        }
    }
}
