#![allow(clippy::all, deprecated)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PublicImportMessage<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub e: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct ImportMessage<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub d: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
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
            Self::ImportFoo => "IMPORT_FOO",
            Self::ImportBar => "IMPORT_BAR",
            Self::ImportBaz => "IMPORT_BAZ",
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
            Self::Unknown => "UNKNOWN",
            Self::Foo => "FOO",
            Self::Bar => "BAR",
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
