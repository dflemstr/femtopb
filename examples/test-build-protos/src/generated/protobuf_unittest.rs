/// This proto includes every type of field in both singular and repeated
/// forms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestAllTypes<'a> {
    /// Singular
    #[femtopb(int32, optional, tag = 1)]
    pub optional_int32: ::core::option::Option<i32>,
    #[femtopb(int64, optional, tag = 2)]
    pub optional_int64: ::core::option::Option<i64>,
    #[femtopb(uint32, optional, tag = 3)]
    pub optional_uint32: ::core::option::Option<u32>,
    #[femtopb(uint64, optional, tag = 4)]
    pub optional_uint64: ::core::option::Option<u64>,
    #[femtopb(sint32, optional, tag = 5)]
    pub optional_sint32: ::core::option::Option<i32>,
    #[femtopb(sint64, optional, tag = 6)]
    pub optional_sint64: ::core::option::Option<i64>,
    #[femtopb(fixed32, optional, tag = 7)]
    pub optional_fixed32: ::core::option::Option<u32>,
    #[femtopb(fixed64, optional, tag = 8)]
    pub optional_fixed64: ::core::option::Option<u64>,
    #[femtopb(sfixed32, optional, tag = 9)]
    pub optional_sfixed32: ::core::option::Option<i32>,
    #[femtopb(sfixed64, optional, tag = 10)]
    pub optional_sfixed64: ::core::option::Option<i64>,
    #[femtopb(float, optional, tag = 11)]
    pub optional_float: ::core::option::Option<f32>,
    #[femtopb(double, optional, tag = 12)]
    pub optional_double: ::core::option::Option<f64>,
    #[femtopb(bool, optional, tag = 13)]
    pub optional_bool: ::core::option::Option<bool>,
    #[femtopb(string, optional, tag = 14)]
    pub optional_string: ::core::option::Option<&'a str>,
    #[femtopb(bytes, optional, tag = 15)]
    pub optional_bytes: ::core::option::Option<&'a [u8]>,
    #[femtopb(message, optional, tag = 18)]
    pub optional_nested_message: ::core::option::Option<
        test_all_types::NestedMessage<'a>,
    >,
    #[femtopb(message, optional, tag = 19)]
    pub optional_foreign_message: ::core::option::Option<ForeignMessage<'a>>,
    #[femtopb(message, optional, tag = 20)]
    pub optional_import_message: ::core::option::Option<
        super::protobuf_unittest_import::ImportMessage<'a>,
    >,
    #[femtopb(enumeration, optional, tag = 21)]
    pub optional_nested_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<test_all_types::NestedEnum>,
    >,
    #[femtopb(enumeration, optional, tag = 22)]
    pub optional_foreign_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<ForeignEnum>,
    >,
    #[femtopb(enumeration, optional, tag = 23)]
    pub optional_import_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<super::protobuf_unittest_import::ImportEnum>,
    >,
    #[femtopb(string, optional, tag = 24)]
    pub optional_string_piece: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 25)]
    pub optional_cord: ::core::option::Option<&'a str>,
    /// Defined in unittest_import_public.proto
    #[femtopb(message, optional, tag = 26)]
    pub optional_public_import_message: ::core::option::Option<
        super::protobuf_unittest_import::PublicImportMessage<'a>,
    >,
    #[femtopb(message, optional, tag = 27)]
    pub optional_lazy_message: ::core::option::Option<test_all_types::NestedMessage<'a>>,
    #[femtopb(message, optional, tag = 28)]
    pub optional_unverified_lazy_message: ::core::option::Option<
        test_all_types::NestedMessage<'a>,
    >,
    /// Repeated
    #[femtopb(int32, repeated, tag = 31)]
    pub repeated_int32: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    #[femtopb(int64, repeated, tag = 32)]
    pub repeated_int64: ::femtopb::repeated::Repeated<
        'a,
        i64,
        ::femtopb::item_encoding::Int64,
    >,
    #[femtopb(uint32, repeated, tag = 33)]
    pub repeated_uint32: ::femtopb::repeated::Repeated<
        'a,
        u32,
        ::femtopb::item_encoding::UInt32,
    >,
    #[femtopb(uint64, repeated, tag = 34)]
    pub repeated_uint64: ::femtopb::repeated::Repeated<
        'a,
        u64,
        ::femtopb::item_encoding::UInt64,
    >,
    #[femtopb(sint32, repeated, tag = 35)]
    pub repeated_sint32: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::SInt32,
    >,
    #[femtopb(sint64, repeated, tag = 36)]
    pub repeated_sint64: ::femtopb::repeated::Repeated<
        'a,
        i64,
        ::femtopb::item_encoding::SInt64,
    >,
    #[femtopb(fixed32, repeated, tag = 37)]
    pub repeated_fixed32: ::femtopb::repeated::Repeated<
        'a,
        u32,
        ::femtopb::item_encoding::Fixed32,
    >,
    #[femtopb(fixed64, repeated, tag = 38)]
    pub repeated_fixed64: ::femtopb::repeated::Repeated<
        'a,
        u64,
        ::femtopb::item_encoding::Fixed64,
    >,
    #[femtopb(sfixed32, repeated, tag = 39)]
    pub repeated_sfixed32: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::SFixed32,
    >,
    #[femtopb(sfixed64, repeated, tag = 40)]
    pub repeated_sfixed64: ::femtopb::repeated::Repeated<
        'a,
        i64,
        ::femtopb::item_encoding::SFixed64,
    >,
    #[femtopb(float, repeated, tag = 41)]
    pub repeated_float: ::femtopb::repeated::Repeated<
        'a,
        f32,
        ::femtopb::item_encoding::Float,
    >,
    #[femtopb(double, repeated, tag = 42)]
    pub repeated_double: ::femtopb::repeated::Repeated<
        'a,
        f64,
        ::femtopb::item_encoding::Double,
    >,
    #[femtopb(bool, repeated, tag = 43)]
    pub repeated_bool: ::femtopb::repeated::Repeated<
        'a,
        bool,
        ::femtopb::item_encoding::Bool,
    >,
    #[femtopb(string, repeated, tag = 44)]
    pub repeated_string: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(bytes, repeated, tag = 45)]
    pub repeated_bytes: ::femtopb::repeated::Repeated<
        'a,
        &'a [u8],
        ::femtopb::item_encoding::Bytes,
    >,
    #[femtopb(message, repeated, tag = 48)]
    pub repeated_nested_message: ::femtopb::repeated::Repeated<
        'a,
        test_all_types::NestedMessage<'a>,
        ::femtopb::item_encoding::Message<'a, test_all_types::NestedMessage<'a>>,
    >,
    #[femtopb(message, repeated, tag = 49)]
    pub repeated_foreign_message: ::femtopb::repeated::Repeated<
        'a,
        ForeignMessage<'a>,
        ::femtopb::item_encoding::Message<'a, ForeignMessage<'a>>,
    >,
    #[femtopb(message, repeated, tag = 50)]
    pub repeated_import_message: ::femtopb::repeated::Repeated<
        'a,
        super::protobuf_unittest_import::ImportMessage<'a>,
        ::femtopb::item_encoding::Message<
            'a,
            super::protobuf_unittest_import::ImportMessage<'a>,
        >,
    >,
    #[femtopb(enumeration, repeated, tag = 51)]
    pub repeated_nested_enum: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<test_all_types::NestedEnum>,
        ::femtopb::item_encoding::Enum<test_all_types::NestedEnum>,
    >,
    #[femtopb(enumeration, repeated, tag = 52)]
    pub repeated_foreign_enum: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<ForeignEnum>,
        ::femtopb::item_encoding::Enum<ForeignEnum>,
    >,
    #[femtopb(enumeration, repeated, tag = 53)]
    pub repeated_import_enum: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<super::protobuf_unittest_import::ImportEnum>,
        ::femtopb::item_encoding::Enum<super::protobuf_unittest_import::ImportEnum>,
    >,
    #[femtopb(string, repeated, tag = 54)]
    pub repeated_string_piece: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 55)]
    pub repeated_cord: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(message, repeated, tag = 57)]
    pub repeated_lazy_message: ::femtopb::repeated::Repeated<
        'a,
        test_all_types::NestedMessage<'a>,
        ::femtopb::item_encoding::Message<'a, test_all_types::NestedMessage<'a>>,
    >,
    /// Singular with defaults
    #[femtopb(int32, optional, tag = 61, default = 41)]
    pub default_int32: ::core::option::Option<i32>,
    #[femtopb(int64, optional, tag = 62, default = 42)]
    pub default_int64: ::core::option::Option<i64>,
    #[femtopb(uint32, optional, tag = 63, default = 43)]
    pub default_uint32: ::core::option::Option<u32>,
    #[femtopb(uint64, optional, tag = 64, default = 44)]
    pub default_uint64: ::core::option::Option<u64>,
    #[femtopb(sint32, optional, tag = 65, default = -45)]
    pub default_sint32: ::core::option::Option<i32>,
    #[femtopb(sint64, optional, tag = 66, default = 46)]
    pub default_sint64: ::core::option::Option<i64>,
    #[femtopb(fixed32, optional, tag = 67, default = 47)]
    pub default_fixed32: ::core::option::Option<u32>,
    #[femtopb(fixed64, optional, tag = 68, default = 48)]
    pub default_fixed64: ::core::option::Option<u64>,
    #[femtopb(sfixed32, optional, tag = 69, default = 49)]
    pub default_sfixed32: ::core::option::Option<i32>,
    #[femtopb(sfixed64, optional, tag = 70, default = -50)]
    pub default_sfixed64: ::core::option::Option<i64>,
    #[femtopb(float, optional, tag = 71, default = 51.5)]
    pub default_float: ::core::option::Option<f32>,
    #[femtopb(double, optional, tag = 72, default = 52000)]
    pub default_double: ::core::option::Option<f64>,
    #[femtopb(bool, optional, tag = 73, default = true)]
    pub default_bool: ::core::option::Option<bool>,
    #[femtopb(string, optional, tag = 74, default = "hello")]
    pub default_string: ::core::option::Option<&'a str>,
    #[femtopb(bytes, optional, tag = 75, default = b"world")]
    pub default_bytes: ::core::option::Option<&'a [u8]>,
    #[femtopb(
        enumeration,
        optional,
        tag = 81,
        default = test_all_types::NestedEnum::Bar
    )]
    pub default_nested_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<test_all_types::NestedEnum>,
    >,
    #[femtopb(enumeration, optional, tag = 82, default = ForeignEnum::ForeignBar)]
    pub default_foreign_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<ForeignEnum>,
    >,
    #[femtopb(
        enumeration,
        optional,
        tag = 83,
        default = super::protobuf_unittest_import::ImportEnum::ImportBar
    )]
    pub default_import_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<super::protobuf_unittest_import::ImportEnum>,
    >,
    #[femtopb(string, optional, tag = 84, default = "abc")]
    pub default_string_piece: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 85, default = "123")]
    pub default_cord: ::core::option::Option<&'a str>,
    /// For oneof test
    #[femtopb(oneof, tags = [111, 112, 113, 114, 115, 116, 117])]
    pub oneof_field: ::core::option::Option<test_all_types::OneofField<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestAllTypes`.
pub mod test_all_types {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct NestedMessage<'a> {
        /// The field name "b" fails to compile in proto1 because it conflicts with
        /// a local variable named "b" in one of the generated methods.  Doh.
        /// This file needs to compile in proto1 to test backwards-compatibility.
        #[femtopb(int32, optional, tag = 1)]
        pub bb: ::core::option::Option<i32>,
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
    pub enum NestedEnum {
        #[default]
        Foo = 1,
        Bar = 2,
        Baz = 3,
        /// Intentionally negative.
        Neg = -1,
    }
    impl NestedEnum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NestedEnum::Foo => "FOO",
                NestedEnum::Bar => "BAR",
                NestedEnum::Baz => "BAZ",
                NestedEnum::Neg => "NEG",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FOO" => Some(Self::Foo),
                "BAR" => Some(Self::Bar),
                "BAZ" => Some(Self::Baz),
                "NEG" => Some(Self::Neg),
                _ => None,
            }
        }
    }
    /// For oneof test
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum OneofField<'a> {
        #[femtopb(uint32, tag = 111)]
        OneofUint32(u32),
        #[femtopb(message, tag = 112)]
        OneofNestedMessage(NestedMessage<'a>),
        #[femtopb(string, tag = 113)]
        OneofString(&'a str),
        #[femtopb(bytes, tag = 114)]
        OneofBytes(&'a [u8]),
        #[femtopb(string, tag = 115)]
        OneofCord(&'a str),
        #[femtopb(string, tag = 116)]
        OneofStringPiece(&'a str),
        #[femtopb(message, tag = 117)]
        OneofLazyNestedMessage(NestedMessage<'a>),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
/// This proto includes a recursively nested message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct NestedTestAllTypes<'a> {
    #[femtopb(message, optional, deferred, tag = 1)]
    pub child: ::core::option::Option<
        ::femtopb::deferred::Deferred<'a, NestedTestAllTypes<'a>>,
    >,
    #[femtopb(message, optional, tag = 2)]
    pub payload: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(message, repeated, tag = 3)]
    pub repeated_child: ::femtopb::repeated::Repeated<
        'a,
        NestedTestAllTypes<'a>,
        ::femtopb::item_encoding::Message<'a, NestedTestAllTypes<'a>>,
    >,
    #[femtopb(message, optional, deferred, tag = 4)]
    pub lazy_child: ::core::option::Option<
        ::femtopb::deferred::Deferred<'a, NestedTestAllTypes<'a>>,
    >,
    #[femtopb(message, optional, tag = 5)]
    pub eager_child: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestDeprecatedFields<'a> {
    #[deprecated]
    #[femtopb(int32, optional, tag = 1)]
    pub deprecated_int32: ::core::option::Option<i32>,
    #[deprecated]
    #[femtopb(string, repeated, tag = 4)]
    pub deprecated_repeated_string: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[deprecated]
    #[femtopb(message, optional, tag = 3)]
    pub deprecated_message: ::core::option::Option<test_all_types::NestedMessage<'a>>,
    #[femtopb(message, optional, deferred, tag = 5)]
    pub nested: ::core::option::Option<
        ::femtopb::deferred::Deferred<'a, TestDeprecatedFields<'a>>,
    >,
    #[femtopb(oneof, tags = [2])]
    pub oneof_fields: ::core::option::Option<test_deprecated_fields::OneofFields<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestDeprecatedFields`.
pub mod test_deprecated_fields {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum OneofFields<'a> {
        #[femtopb(int32, tag = 2)]
        DeprecatedInt32InOneof(i32),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestDeprecatedMessage<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Define these after TestAllTypes to make sure the compiler can handle
/// that.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ForeignMessage<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub c: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 2)]
    pub d: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestReservedFields<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestAllExtensions<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestMixedFieldsAndExtensions<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub a: ::core::option::Option<i32>,
    #[femtopb(fixed32, repeated, tag = 3)]
    pub b: ::femtopb::repeated::Repeated<'a, u32, ::femtopb::item_encoding::Fixed32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestGroup<'a> {
    ///   optional group OptionalGroup = 16 {
    ///     optional int32 a = 17;
    ///     optional int32 zz = 89;  // fast table size must be at least 16, for this
    ///                              // field to be parsed by the fast parser, since
    ///                              // 89 - 17 = 72 is a multiple of 8.
    ///   }
    #[femtopb(enumeration, optional, tag = 22)]
    pub optional_foreign_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<ForeignEnum>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestGroupExtension<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestNestedExtension<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestChildExtension<'a> {
    #[femtopb(string, optional, tag = 1)]
    pub a: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 2)]
    pub b: ::core::option::Option<&'a str>,
    #[femtopb(message, optional, tag = 3)]
    pub optional_extension: ::core::option::Option<TestAllExtensions<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Emulates wireformat data of TestChildExtension with dynamic extension
/// (DynamicExtension).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestChildExtensionData<'a> {
    #[femtopb(string, optional, tag = 1)]
    pub a: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 2)]
    pub b: ::core::option::Option<&'a str>,
    #[femtopb(message, optional, tag = 3)]
    pub optional_extension: ::core::option::Option<
        test_child_extension_data::NestedTestAllExtensionsData<'a>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestChildExtensionData`.
pub mod test_child_extension_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct NestedTestAllExtensionsData<'a> {
        #[femtopb(message, optional, tag = 409707008)]
        pub dynamic: ::core::option::Option<
            nested_test_all_extensions_data::NestedDynamicExtensions<'a>,
        >,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    /// Nested message and enum types in `NestedTestAllExtensionsData`.
    pub mod nested_test_all_extensions_data {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::femtopb::Message)]
        pub struct NestedDynamicExtensions<'a> {
            #[femtopb(int32, optional, tag = 1)]
            pub a: ::core::option::Option<i32>,
            #[femtopb(int32, optional, tag = 2)]
            pub b: ::core::option::Option<i32>,
            #[femtopb(unknown_fields)]
            pub unknown_fields: femtopb::UnknownFields<'a>,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestNestedChildExtension<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub a: ::core::option::Option<i32>,
    #[femtopb(message, optional, tag = 2)]
    pub child: ::core::option::Option<TestChildExtension<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Emulates wireformat data of TestNestedChildExtension with dynamic extension
/// (DynamicExtension).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestNestedChildExtensionData<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub a: ::core::option::Option<i32>,
    #[femtopb(message, optional, tag = 2)]
    pub child: ::core::option::Option<TestChildExtensionData<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Required and closed enum fields are considered unknown fields if the value is
/// not valid. We need to make sure it functions as expected.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestRequiredEnum<'a> {
    #[femtopb(enumeration, required, tag = 1)]
    pub required_enum: ::femtopb::enumeration::EnumValue<ForeignEnum>,
    /// A dummy optional field.
    #[femtopb(int32, optional, tag = 2)]
    pub a: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// TestRequiredEnum + using enum values that won't fit to 64 bitmask.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestRequiredEnumNoMask<'a> {
    #[femtopb(enumeration, required, tag = 1)]
    pub required_enum: ::femtopb::enumeration::EnumValue<
        test_required_enum_no_mask::NestedEnum,
    >,
    /// A dummy optional field.
    #[femtopb(int32, optional, tag = 2)]
    pub a: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestRequiredEnumNoMask`.
pub mod test_required_enum_no_mask {
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
    pub enum NestedEnum {
        #[default]
        Unspecified = 0,
        Foo = 2,
        Bar = 100,
        /// Intentionally negative.
        Baz = -1,
    }
    impl NestedEnum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NestedEnum::Unspecified => "UNSPECIFIED",
                NestedEnum::Foo => "FOO",
                NestedEnum::Bar => "BAR",
                NestedEnum::Baz => "BAZ",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "FOO" => Some(Self::Foo),
                "BAR" => Some(Self::Bar),
                "BAZ" => Some(Self::Baz),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestRequiredEnumMulti<'a> {
    /// Intentionally placed in descending field number to force sorting in closed
    /// enum verification.
    #[femtopb(enumeration, required, tag = 4)]
    pub required_enum_4: ::femtopb::enumeration::EnumValue<
        test_required_enum_multi::NestedEnum,
    >,
    #[femtopb(int32, optional, tag = 3)]
    pub a_3: ::core::option::Option<i32>,
    #[femtopb(enumeration, required, tag = 2)]
    pub required_enum_2: ::femtopb::enumeration::EnumValue<
        test_required_enum_multi::NestedEnum,
    >,
    #[femtopb(enumeration, required, tag = 1)]
    pub required_enum_1: ::femtopb::enumeration::EnumValue<ForeignEnum>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestRequiredEnumMulti`.
pub mod test_required_enum_multi {
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
    pub enum NestedEnum {
        #[default]
        Unspecified = 0,
        Foo = 1,
        Bar = 2,
        Baz = 100,
    }
    impl NestedEnum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NestedEnum::Unspecified => "UNSPECIFIED",
                NestedEnum::Foo => "FOO",
                NestedEnum::Bar => "BAR",
                NestedEnum::Baz => "BAZ",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "FOO" => Some(Self::Foo),
                "BAR" => Some(Self::Bar),
                "BAZ" => Some(Self::Baz),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestRequiredNoMaskMulti<'a> {
    /// Intentionally placed in descending field number to force sorting in closed
    /// enum verification. Also, using large field numbers to use tag only
    /// matching for required fields.
    #[femtopb(fixed32, required, tag = 80)]
    pub required_fixed32_80: u32,
    #[femtopb(fixed32, required, tag = 70)]
    pub required_fixed32_70: u32,
    #[femtopb(enumeration, required, tag = 64)]
    pub required_enum_64: ::femtopb::enumeration::EnumValue<
        test_required_no_mask_multi::NestedEnum,
    >,
    #[femtopb(enumeration, required, tag = 4)]
    pub required_enum_4: ::femtopb::enumeration::EnumValue<
        test_required_no_mask_multi::NestedEnum,
    >,
    #[femtopb(int32, optional, tag = 3)]
    pub a_3: ::core::option::Option<i32>,
    #[femtopb(enumeration, required, tag = 2)]
    pub required_enum_2: ::femtopb::enumeration::EnumValue<
        test_required_no_mask_multi::NestedEnum,
    >,
    #[femtopb(enumeration, required, tag = 1)]
    pub required_enum_1: ::femtopb::enumeration::EnumValue<ForeignEnum>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestRequiredNoMaskMulti`.
pub mod test_required_no_mask_multi {
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
    pub enum NestedEnum {
        #[default]
        Unspecified = 0,
        Foo = 1,
        Bar = 2,
        Baz = 100,
    }
    impl NestedEnum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NestedEnum::Unspecified => "UNSPECIFIED",
                NestedEnum::Foo => "FOO",
                NestedEnum::Bar => "BAR",
                NestedEnum::Baz => "BAZ",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "FOO" => Some(Self::Foo),
                "BAR" => Some(Self::Bar),
                "BAZ" => Some(Self::Baz),
                _ => None,
            }
        }
    }
}
/// We have separate messages for testing required fields because it's
/// annoying to have to fill in required fields in TestProto in order to
/// do anything with it.  Note that we don't need to test every type of
/// required filed because the code output is basically identical to
/// optional fields for all types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestRequired<'a> {
    #[femtopb(int32, required, tag = 1)]
    pub a: i32,
    #[femtopb(int32, optional, tag = 2)]
    pub dummy2: ::core::option::Option<i32>,
    #[femtopb(int32, required, tag = 3)]
    pub b: i32,
    /// Pad the field count to 32 so that we can test that IsInitialized()
    /// properly checks multiple elements of has_bits_.
    #[femtopb(int32, optional, tag = 4)]
    pub dummy4: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 5)]
    pub dummy5: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 6)]
    pub dummy6: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 7)]
    pub dummy7: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 8)]
    pub dummy8: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 9)]
    pub dummy9: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 10)]
    pub dummy10: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 11)]
    pub dummy11: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 12)]
    pub dummy12: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 13)]
    pub dummy13: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 14)]
    pub dummy14: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 15)]
    pub dummy15: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 16)]
    pub dummy16: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 17)]
    pub dummy17: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 18)]
    pub dummy18: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 19)]
    pub dummy19: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 20)]
    pub dummy20: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 21)]
    pub dummy21: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 22)]
    pub dummy22: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 23)]
    pub dummy23: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 24)]
    pub dummy24: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 25)]
    pub dummy25: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 26)]
    pub dummy26: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 27)]
    pub dummy27: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 28)]
    pub dummy28: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 29)]
    pub dummy29: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 30)]
    pub dummy30: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 31)]
    pub dummy31: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 32)]
    pub dummy32: ::core::option::Option<i32>,
    #[femtopb(int32, required, tag = 33)]
    pub c: i32,
    /// Add an optional child message to make this non-trivial for go/pdlazy.
    #[femtopb(message, optional, tag = 34)]
    pub optional_foreign: ::core::option::Option<ForeignMessage<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestRequiredForeign<'a> {
    #[femtopb(message, optional, tag = 1)]
    pub optional_message: ::core::option::Option<TestRequired<'a>>,
    #[femtopb(message, repeated, tag = 2)]
    pub repeated_message: ::femtopb::repeated::Repeated<
        'a,
        TestRequired<'a>,
        ::femtopb::item_encoding::Message<'a, TestRequired<'a>>,
    >,
    #[femtopb(int32, optional, tag = 3)]
    pub dummy: ::core::option::Option<i32>,
    /// Missing required fields must not affect verification of child messages.
    #[femtopb(message, optional, tag = 4)]
    pub optional_lazy_message: ::core::option::Option<NestedTestAllTypes<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestRequiredMessage<'a> {
    #[femtopb(message, optional, tag = 1)]
    pub optional_message: ::core::option::Option<TestRequired<'a>>,
    #[femtopb(message, repeated, tag = 2)]
    pub repeated_message: ::femtopb::repeated::Repeated<
        'a,
        TestRequired<'a>,
        ::femtopb::item_encoding::Message<'a, TestRequired<'a>>,
    >,
    #[femtopb(message, required, tag = 3)]
    pub required_message: TestRequired<'a>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestNestedRequiredForeign<'a> {
    #[femtopb(message, optional, deferred, tag = 1)]
    pub child: ::core::option::Option<
        ::femtopb::deferred::Deferred<'a, TestNestedRequiredForeign<'a>>,
    >,
    #[femtopb(message, optional, tag = 2)]
    pub payload: ::core::option::Option<TestRequiredForeign<'a>>,
    #[femtopb(int32, optional, tag = 3)]
    pub dummy: ::core::option::Option<i32>,
    /// optional message to test required closed enum.
    #[femtopb(message, optional, tag = 5)]
    pub required_enum: ::core::option::Option<TestRequiredEnum<'a>>,
    #[femtopb(message, optional, tag = 6)]
    pub required_enum_no_mask: ::core::option::Option<TestRequiredEnumNoMask<'a>>,
    #[femtopb(message, optional, tag = 7)]
    pub required_enum_multi: ::core::option::Option<TestRequiredEnumMulti<'a>>,
    #[femtopb(message, optional, tag = 9)]
    pub required_no_mask: ::core::option::Option<TestRequiredNoMaskMulti<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Test that we can use NestedMessage from outside TestAllTypes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestForeignNested<'a> {
    #[femtopb(message, optional, tag = 1)]
    pub foreign_nested: ::core::option::Option<test_all_types::NestedMessage<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// TestEmptyMessage is used to test unknown field support.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestEmptyMessage<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Like above, but declare all field numbers as potential extensions.  No
/// actual extensions should ever be defined for this type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestEmptyMessageWithExtensions<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Needed for a Python test.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestPickleNestedMessage<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestPickleNestedMessage`.
pub mod test_pickle_nested_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct NestedMessage<'a> {
        #[femtopb(int32, optional, tag = 1)]
        pub bb: ::core::option::Option<i32>,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    /// Nested message and enum types in `NestedMessage`.
    pub mod nested_message {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::femtopb::Message)]
        pub struct NestedNestedMessage<'a> {
            #[femtopb(int32, optional, tag = 1)]
            pub cc: ::core::option::Option<i32>,
            #[femtopb(unknown_fields)]
            pub unknown_fields: femtopb::UnknownFields<'a>,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestMultipleExtensionRanges<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Test that really large tag numbers don't break anything.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestReallyLargeTagNumber<'a> {
    /// The largest possible tag number is 2^28 - 1, since the wire format uses
    /// three bits to communicate wire type.
    #[femtopb(int32, optional, tag = 1)]
    pub a: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 268435455)]
    pub bb: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestRecursiveMessage<'a> {
    #[femtopb(message, optional, deferred, tag = 1)]
    pub a: ::core::option::Option<
        ::femtopb::deferred::Deferred<'a, TestRecursiveMessage<'a>>,
    >,
    #[femtopb(int32, optional, tag = 2)]
    pub i: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Test that mutual recursion works.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestMutualRecursionA<'a> {
    ///   optional group SubGroup = 2 {
    ///     optional SubMessage sub_message = 3;  // Needed because of bug in javatest
    ///     optional TestAllTypes not_in_this_scc = 4;
    ///   }
    ///   repeated group SubGroupR = 5 {
    ///     optional TestAllTypes payload = 6;
    ///   }
    #[femtopb(message, optional, deferred, tag = 1)]
    pub bb: ::core::option::Option<
        ::femtopb::deferred::Deferred<'a, TestMutualRecursionB<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestMutualRecursionA`.
pub mod test_mutual_recursion_a {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct SubMessage<'a> {
        #[femtopb(message, optional, tag = 1)]
        pub b: ::core::option::Option<super::TestMutualRecursionB<'a>>,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestMutualRecursionB<'a> {
    #[femtopb(message, optional, deferred, tag = 1)]
    pub a: ::core::option::Option<
        ::femtopb::deferred::Deferred<'a, TestMutualRecursionA<'a>>,
    >,
    #[femtopb(int32, optional, tag = 2)]
    pub optional_int32: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestIsInitialized<'a> {
    #[femtopb(message, optional, tag = 1)]
    pub sub_message: ::core::option::Option<test_is_initialized::SubMessage<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestIsInitialized`.
pub mod test_is_initialized {
    ///     optional group SubGroup = 1 {
    ///       required int32 i = 2;
    ///     }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct SubMessage<'a> {
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
}
/// Test that groups have disjoint field numbers from their siblings and
/// parents.  This is NOT possible in proto1; only google.protobuf.  When attempting
/// to compile with proto1, this will emit an error; so we only include it
/// in protobuf_unittest_proto.
///
/// NO_PROTO1
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestDupFieldNumber<'a> {
    /// NO_PROTO1
    #[femtopb(int32, optional, tag = 1)]
    pub a: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Additional messages for testing lazy fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestEagerMessage<'a> {
    #[femtopb(message, optional, tag = 1)]
    pub sub_message: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestLazyMessage<'a> {
    #[femtopb(message, optional, tag = 1)]
    pub sub_message: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestLazyMessageRepeated<'a> {
    #[femtopb(message, repeated, tag = 1)]
    pub repeated_message: ::femtopb::repeated::Repeated<
        'a,
        TestLazyMessage<'a>,
        ::femtopb::item_encoding::Message<'a, TestLazyMessage<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestEagerMaybeLazy<'a> {
    #[femtopb(message, optional, tag = 1)]
    pub message_foo: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(message, optional, tag = 2)]
    pub message_bar: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(message, optional, tag = 3)]
    pub message_baz: ::core::option::Option<test_eager_maybe_lazy::NestedMessage<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestEagerMaybeLazy`.
pub mod test_eager_maybe_lazy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct NestedMessage<'a> {
        #[femtopb(message, optional, tag = 1)]
        pub packed: ::core::option::Option<super::TestPackedTypes<'a>>,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
}
/// Needed for a Python test.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestNestedMessageHasBits<'a> {
    #[femtopb(message, optional, tag = 1)]
    pub optional_nested_message: ::core::option::Option<
        test_nested_message_has_bits::NestedMessage<'a>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestNestedMessageHasBits`.
pub mod test_nested_message_has_bits {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct NestedMessage<'a> {
        #[femtopb(int32, repeated, tag = 1)]
        pub nestedmessage_repeated_int32: ::femtopb::repeated::Repeated<
            'a,
            i32,
            ::femtopb::item_encoding::Int32,
        >,
        #[femtopb(message, repeated, tag = 2)]
        pub nestedmessage_repeated_foreignmessage: ::femtopb::repeated::Repeated<
            'a,
            super::ForeignMessage<'a>,
            ::femtopb::item_encoding::Message<'a, super::ForeignMessage<'a>>,
        >,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
}
/// Test message with CamelCase field names.  This violates Protocol Buffer
/// standard style.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestCamelCaseFieldNames<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub primitive_field: ::core::option::Option<i32>,
    #[femtopb(string, optional, tag = 2)]
    pub string_field: ::core::option::Option<&'a str>,
    #[femtopb(enumeration, optional, tag = 3)]
    pub enum_field: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<ForeignEnum>,
    >,
    #[femtopb(message, optional, tag = 4)]
    pub message_field: ::core::option::Option<ForeignMessage<'a>>,
    #[femtopb(string, optional, tag = 5)]
    pub string_piece_field: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 6)]
    pub cord_field: ::core::option::Option<&'a str>,
    #[femtopb(int32, repeated, tag = 7)]
    pub repeated_primitive_field: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    #[femtopb(string, repeated, tag = 8)]
    pub repeated_string_field: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(enumeration, repeated, tag = 9)]
    pub repeated_enum_field: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<ForeignEnum>,
        ::femtopb::item_encoding::Enum<ForeignEnum>,
    >,
    #[femtopb(message, repeated, tag = 10)]
    pub repeated_message_field: ::femtopb::repeated::Repeated<
        'a,
        ForeignMessage<'a>,
        ::femtopb::item_encoding::Message<'a, ForeignMessage<'a>>,
    >,
    #[femtopb(string, repeated, tag = 11)]
    pub repeated_string_piece_field: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 12)]
    pub repeated_cord_field: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// We list fields out of order, to ensure that we're using field number and not
/// field index to determine serialization order.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestFieldOrderings<'a> {
    #[femtopb(string, optional, tag = 11)]
    pub my_string: ::core::option::Option<&'a str>,
    #[femtopb(int64, optional, tag = 1)]
    pub my_int: ::core::option::Option<i64>,
    #[femtopb(float, optional, tag = 101)]
    pub my_float: ::core::option::Option<f32>,
    #[femtopb(message, optional, tag = 200)]
    pub optional_nested_message: ::core::option::Option<
        test_field_orderings::NestedMessage<'a>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestFieldOrderings`.
pub mod test_field_orderings {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct NestedMessage<'a> {
        #[femtopb(int64, optional, tag = 2)]
        pub oo: ::core::option::Option<i64>,
        /// The field name "b" fails to compile in proto1 because it conflicts with
        /// a local variable named "b" in one of the generated methods.  Doh.
        /// This file needs to compile in proto1 to test backwards-compatibility.
        #[femtopb(int32, optional, tag = 1)]
        pub bb: ::core::option::Option<i32>,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestExtensionOrderings1<'a> {
    #[femtopb(string, optional, tag = 1)]
    pub my_string: ::core::option::Option<&'a str>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestExtensionOrderings2<'a> {
    #[femtopb(string, optional, tag = 1)]
    pub my_string: ::core::option::Option<&'a str>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestExtensionOrderings2`.
pub mod test_extension_orderings2 {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct TestExtensionOrderings3<'a> {
        #[femtopb(string, optional, tag = 1)]
        pub my_string: ::core::option::Option<&'a str>,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestExtremeDefaultValues<'a> {
    #[femtopb(
        bytes,
        optional,
        tag = 1,
        default = b"\x00\x01\x07\x08\x0c\n\r\t\x0b\\\'\"\xfe"
    )]
    pub escaped_bytes: ::core::option::Option<&'a [u8]>,
    #[femtopb(uint32, optional, tag = 2, default = 4294967295)]
    pub large_uint32: ::core::option::Option<u32>,
    #[femtopb(uint64, optional, tag = 3, default = 18446744073709551615)]
    pub large_uint64: ::core::option::Option<u64>,
    #[femtopb(int32, optional, tag = 4, default = -2147483647)]
    pub small_int32: ::core::option::Option<i32>,
    #[femtopb(int64, optional, tag = 5, default = -9223372036854775807)]
    pub small_int64: ::core::option::Option<i64>,
    #[femtopb(int32, optional, tag = 21, default = -2147483648)]
    pub really_small_int32: ::core::option::Option<i32>,
    #[femtopb(int64, optional, tag = 22, default = -9223372036854775808)]
    pub really_small_int64: ::core::option::Option<i64>,
    /// The default value here is UTF-8 for "\u1234".  (We could also just type
    /// the UTF-8 text directly into this text file rather than escape it, but
    /// lots of people use editors that would be confused by this.)
    #[femtopb(string, optional, tag = 6, default = "ሴ")]
    pub utf8_string: ::core::option::Option<&'a str>,
    /// Tests for single-precision floating-point values.
    #[femtopb(float, optional, tag = 7, default = 0)]
    pub zero_float: ::core::option::Option<f32>,
    #[femtopb(float, optional, tag = 8, default = 1)]
    pub one_float: ::core::option::Option<f32>,
    #[femtopb(float, optional, tag = 9, default = 1.5)]
    pub small_float: ::core::option::Option<f32>,
    #[femtopb(float, optional, tag = 10, default = -1)]
    pub negative_one_float: ::core::option::Option<f32>,
    #[femtopb(float, optional, tag = 11, default = -1.5)]
    pub negative_float: ::core::option::Option<f32>,
    /// Using exponents
    #[femtopb(float, optional, tag = 12, default = 200000000)]
    pub large_float: ::core::option::Option<f32>,
    #[femtopb(float, optional, tag = 13, default = -0.0000000000000000000000000008)]
    pub small_negative_float: ::core::option::Option<f32>,
    /// Text for nonfinite floating-point values.
    #[femtopb(double, optional, tag = 14, default = inf)]
    pub inf_double: ::core::option::Option<f64>,
    #[femtopb(double, optional, tag = 15, default = -inf)]
    pub neg_inf_double: ::core::option::Option<f64>,
    #[femtopb(double, optional, tag = 16, default = nan)]
    pub nan_double: ::core::option::Option<f64>,
    #[femtopb(float, optional, tag = 17, default = inf)]
    pub inf_float: ::core::option::Option<f32>,
    #[femtopb(float, optional, tag = 18, default = -inf)]
    pub neg_inf_float: ::core::option::Option<f32>,
    #[femtopb(float, optional, tag = 19, default = nan)]
    pub nan_float: ::core::option::Option<f32>,
    /// Tests for C++ trigraphs.
    /// Trigraphs should be escaped in C++ generated files, but they should not be
    /// escaped for other languages.
    /// Note that in .proto file, "\?" is a valid way to escape ? in string
    /// literals.
    #[femtopb(string, optional, tag = 20, default = "? ? ?? ?? ??? ??/ ??-")]
    pub cpp_trigraph: ::core::option::Option<&'a str>,
    /// String defaults containing the character '\000'
    #[femtopb(string, optional, tag = 23, default = "hel\0lo")]
    pub string_with_zero: ::core::option::Option<&'a str>,
    #[femtopb(bytes, optional, tag = 24, default = b"wor\x00ld")]
    pub bytes_with_zero: ::core::option::Option<&'a [u8]>,
    #[femtopb(string, optional, tag = 25, default = "ab\0c")]
    pub string_piece_with_zero: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 26, default = "12\03")]
    pub cord_with_zero: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 27, default = "${unknown}")]
    pub replacement_string: ::core::option::Option<&'a str>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct SparseEnumMessage<'a> {
    #[femtopb(enumeration, optional, tag = 1)]
    pub sparse_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<TestSparseEnum>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Test String and Bytes: string is for valid UTF-8 strings
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct OneString<'a> {
    #[femtopb(string, optional, tag = 1)]
    pub data: ::core::option::Option<&'a str>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct MoreString<'a> {
    #[femtopb(string, repeated, tag = 1)]
    pub data: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct OneBytes<'a> {
    #[femtopb(bytes, optional, tag = 1)]
    pub data: ::core::option::Option<&'a [u8]>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct MoreBytes<'a> {
    #[femtopb(bytes, repeated, tag = 1)]
    pub data: ::femtopb::repeated::Repeated<
        'a,
        &'a [u8],
        ::femtopb::item_encoding::Bytes,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ManyOptionalString<'a> {
    #[femtopb(string, optional, tag = 1)]
    pub str1: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 2)]
    pub str2: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 3)]
    pub str3: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 4)]
    pub str4: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 5)]
    pub str5: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 6)]
    pub str6: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 7)]
    pub str7: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 8)]
    pub str8: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 9)]
    pub str9: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 10)]
    pub str10: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 11)]
    pub str11: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 12)]
    pub str12: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 13)]
    pub str13: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 14)]
    pub str14: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 15)]
    pub str15: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 16)]
    pub str16: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 17)]
    pub str17: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 18)]
    pub str18: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 19)]
    pub str19: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 20)]
    pub str20: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 21)]
    pub str21: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 22)]
    pub str22: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 23)]
    pub str23: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 24)]
    pub str24: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 25)]
    pub str25: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 26)]
    pub str26: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 27)]
    pub str27: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 28)]
    pub str28: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 29)]
    pub str29: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 30)]
    pub str30: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 31)]
    pub str31: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 32)]
    pub str32: ::core::option::Option<&'a str>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Test int32, uint32, int64, uint64, and bool are all compatible
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Int32Message<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub data: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Uint32Message<'a> {
    #[femtopb(uint32, optional, tag = 1)]
    pub data: ::core::option::Option<u32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Int64Message<'a> {
    #[femtopb(int64, optional, tag = 1)]
    pub data: ::core::option::Option<i64>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Uint64Message<'a> {
    #[femtopb(uint64, optional, tag = 1)]
    pub data: ::core::option::Option<u64>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BoolMessage<'a> {
    #[femtopb(bool, optional, tag = 1)]
    pub data: ::core::option::Option<bool>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Test oneofs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestOneof<'a> {
    #[femtopb(oneof, tags = [1, 2, 3])]
    pub foo: ::core::option::Option<test_oneof::Foo<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestOneof`.
pub mod test_oneof {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum Foo<'a> {
        #[femtopb(int32, tag = 1)]
        FooInt(i32),
        #[femtopb(string, tag = 2)]
        FooString(&'a str),
        ///     group FooGroup = 4 {
        ///       optional int32 a = 5;
        ///       optional string b = 6;
        ///     }
        #[femtopb(message, tag = 3)]
        FooMessage(super::TestAllTypes<'a>),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestOneofBackwardsCompatible<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub foo_int: ::core::option::Option<i32>,
    #[femtopb(string, optional, tag = 2)]
    pub foo_string: ::core::option::Option<&'a str>,
    ///   optional group FooGroup = 4 {
    ///     optional int32 a = 5;
    ///     optional string b = 6;
    ///   }
    #[femtopb(message, optional, tag = 3)]
    pub foo_message: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestOneof2<'a> {
    #[femtopb(int32, optional, tag = 18)]
    pub baz_int: ::core::option::Option<i32>,
    #[femtopb(string, optional, tag = 19, default = "BAZ")]
    pub baz_string: ::core::option::Option<&'a str>,
    #[femtopb(oneof, tags = [1, 2, 3, 4, 5, 6, 7, 11, 30])]
    pub foo: ::core::option::Option<test_oneof2::Foo<'a>>,
    #[femtopb(oneof, tags = [12, 13, 14, 15, 16, 17, 20, 21, 22, 23])]
    pub bar: ::core::option::Option<test_oneof2::Bar<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestOneof2`.
pub mod test_oneof2 {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct NestedMessage<'a> {
        #[femtopb(int64, optional, tag = 1)]
        pub moo_int: ::core::option::Option<i64>,
        #[femtopb(int32, repeated, tag = 2)]
        pub corge_int: ::femtopb::repeated::Repeated<
            'a,
            i32,
            ::femtopb::item_encoding::Int32,
        >,
        #[femtopb(message, optional, deferred, tag = 3)]
        pub child: ::core::option::Option<
            ::femtopb::deferred::Deferred<'a, NestedMessage<'a>>,
        >,
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
    pub enum NestedEnum {
        #[default]
        Foo = 1,
        Bar = 2,
        Baz = 3,
    }
    impl NestedEnum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NestedEnum::Foo => "FOO",
                NestedEnum::Bar => "BAR",
                NestedEnum::Baz => "BAZ",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FOO" => Some(Self::Foo),
                "BAR" => Some(Self::Bar),
                "BAZ" => Some(Self::Baz),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum Foo<'a> {
        #[femtopb(int32, tag = 1)]
        FooInt(i32),
        #[femtopb(string, tag = 2)]
        FooString(&'a str),
        #[femtopb(string, tag = 3)]
        FooCord(&'a str),
        #[femtopb(string, tag = 4)]
        FooStringPiece(&'a str),
        #[femtopb(bytes, tag = 5)]
        FooBytes(&'a [u8]),
        #[femtopb(enumeration, tag = 6)]
        FooEnum(::femtopb::enumeration::EnumValue<NestedEnum>),
        #[femtopb(message, tag = 7)]
        FooMessage(NestedMessage<'a>),
        ///     group FooGroup = 8 {
        ///       optional int32 a = 9;
        ///       optional string b = 10;
        ///     }
        #[femtopb(message, tag = 11)]
        FooLazyMessage(NestedMessage<'a>),
        #[femtopb(bytes, tag = 30)]
        FooBytesCord(&'a [u8]),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum Bar<'a> {
        #[femtopb(int32, tag = 12)]
        BarInt(i32),
        #[femtopb(string, tag = 13)]
        BarString(&'a str),
        #[femtopb(string, tag = 14)]
        BarCord(&'a str),
        #[femtopb(string, tag = 15)]
        BarStringPiece(&'a str),
        #[femtopb(bytes, tag = 16)]
        BarBytes(&'a [u8]),
        #[femtopb(enumeration, tag = 17)]
        BarEnum(::femtopb::enumeration::EnumValue<NestedEnum>),
        #[femtopb(string, tag = 20)]
        BarStringWithEmptyDefault(&'a str),
        #[femtopb(string, tag = 21)]
        BarCordWithEmptyDefault(&'a str),
        #[femtopb(string, tag = 22)]
        BarStringPieceWithEmptyDefault(&'a str),
        #[femtopb(bytes, tag = 23)]
        BarBytesWithEmptyDefault(&'a [u8]),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestRequiredOneof<'a> {
    #[femtopb(oneof, tags = [1, 2, 3, 4])]
    pub foo: ::core::option::Option<test_required_oneof::Foo<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestRequiredOneof`.
pub mod test_required_oneof {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct NestedMessage<'a> {
        #[femtopb(double, required, tag = 1)]
        pub required_double: f64,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum Foo<'a> {
        #[femtopb(int32, tag = 1)]
        FooInt(i32),
        #[femtopb(string, tag = 2)]
        FooString(&'a str),
        #[femtopb(message, tag = 3)]
        FooMessage(NestedMessage<'a>),
        #[femtopb(message, tag = 4)]
        FooLazyMessage(NestedMessage<'a>),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestPackedTypes<'a> {
    #[femtopb(int32, packed, tag = 90)]
    pub packed_int32: ::femtopb::packed::Packed<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    #[femtopb(int64, packed, tag = 91)]
    pub packed_int64: ::femtopb::packed::Packed<
        'a,
        i64,
        ::femtopb::item_encoding::Int64,
    >,
    #[femtopb(uint32, packed, tag = 92)]
    pub packed_uint32: ::femtopb::packed::Packed<
        'a,
        u32,
        ::femtopb::item_encoding::UInt32,
    >,
    #[femtopb(uint64, packed, tag = 93)]
    pub packed_uint64: ::femtopb::packed::Packed<
        'a,
        u64,
        ::femtopb::item_encoding::UInt64,
    >,
    #[femtopb(sint32, packed, tag = 94)]
    pub packed_sint32: ::femtopb::packed::Packed<
        'a,
        i32,
        ::femtopb::item_encoding::SInt32,
    >,
    #[femtopb(sint64, packed, tag = 95)]
    pub packed_sint64: ::femtopb::packed::Packed<
        'a,
        i64,
        ::femtopb::item_encoding::SInt64,
    >,
    #[femtopb(fixed32, packed, tag = 96)]
    pub packed_fixed32: ::femtopb::packed::Packed<
        'a,
        u32,
        ::femtopb::item_encoding::Fixed32,
    >,
    #[femtopb(fixed64, packed, tag = 97)]
    pub packed_fixed64: ::femtopb::packed::Packed<
        'a,
        u64,
        ::femtopb::item_encoding::Fixed64,
    >,
    #[femtopb(sfixed32, packed, tag = 98)]
    pub packed_sfixed32: ::femtopb::packed::Packed<
        'a,
        i32,
        ::femtopb::item_encoding::SFixed32,
    >,
    #[femtopb(sfixed64, packed, tag = 99)]
    pub packed_sfixed64: ::femtopb::packed::Packed<
        'a,
        i64,
        ::femtopb::item_encoding::SFixed64,
    >,
    #[femtopb(float, packed, tag = 100)]
    pub packed_float: ::femtopb::packed::Packed<
        'a,
        f32,
        ::femtopb::item_encoding::Float,
    >,
    #[femtopb(double, packed, tag = 101)]
    pub packed_double: ::femtopb::packed::Packed<
        'a,
        f64,
        ::femtopb::item_encoding::Double,
    >,
    #[femtopb(bool, packed, tag = 102)]
    pub packed_bool: ::femtopb::packed::Packed<'a, bool, ::femtopb::item_encoding::Bool>,
    #[femtopb(enumeration, packed, tag = 103)]
    pub packed_enum: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<ForeignEnum>,
        ::femtopb::item_encoding::Enum<ForeignEnum>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// A message with the same fields as TestPackedTypes, but without packing. Used
/// to test packed <-> unpacked wire compatibility.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestUnpackedTypes<'a> {
    #[femtopb(int32, repeated, tag = 90)]
    pub unpacked_int32: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    #[femtopb(int64, repeated, tag = 91)]
    pub unpacked_int64: ::femtopb::repeated::Repeated<
        'a,
        i64,
        ::femtopb::item_encoding::Int64,
    >,
    #[femtopb(uint32, repeated, tag = 92)]
    pub unpacked_uint32: ::femtopb::repeated::Repeated<
        'a,
        u32,
        ::femtopb::item_encoding::UInt32,
    >,
    #[femtopb(uint64, repeated, tag = 93)]
    pub unpacked_uint64: ::femtopb::repeated::Repeated<
        'a,
        u64,
        ::femtopb::item_encoding::UInt64,
    >,
    #[femtopb(sint32, repeated, tag = 94)]
    pub unpacked_sint32: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::SInt32,
    >,
    #[femtopb(sint64, repeated, tag = 95)]
    pub unpacked_sint64: ::femtopb::repeated::Repeated<
        'a,
        i64,
        ::femtopb::item_encoding::SInt64,
    >,
    #[femtopb(fixed32, repeated, tag = 96)]
    pub unpacked_fixed32: ::femtopb::repeated::Repeated<
        'a,
        u32,
        ::femtopb::item_encoding::Fixed32,
    >,
    #[femtopb(fixed64, repeated, tag = 97)]
    pub unpacked_fixed64: ::femtopb::repeated::Repeated<
        'a,
        u64,
        ::femtopb::item_encoding::Fixed64,
    >,
    #[femtopb(sfixed32, repeated, tag = 98)]
    pub unpacked_sfixed32: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::SFixed32,
    >,
    #[femtopb(sfixed64, repeated, tag = 99)]
    pub unpacked_sfixed64: ::femtopb::repeated::Repeated<
        'a,
        i64,
        ::femtopb::item_encoding::SFixed64,
    >,
    #[femtopb(float, repeated, tag = 100)]
    pub unpacked_float: ::femtopb::repeated::Repeated<
        'a,
        f32,
        ::femtopb::item_encoding::Float,
    >,
    #[femtopb(double, repeated, tag = 101)]
    pub unpacked_double: ::femtopb::repeated::Repeated<
        'a,
        f64,
        ::femtopb::item_encoding::Double,
    >,
    #[femtopb(bool, repeated, tag = 102)]
    pub unpacked_bool: ::femtopb::repeated::Repeated<
        'a,
        bool,
        ::femtopb::item_encoding::Bool,
    >,
    #[femtopb(enumeration, repeated, tag = 103)]
    pub unpacked_enum: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<ForeignEnum>,
        ::femtopb::item_encoding::Enum<ForeignEnum>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestPackedExtensions<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestUnpackedExtensions<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Used by ExtensionSetTest/DynamicExtensions.  The test actually builds
/// a set of extensions to TestAllExtensions dynamically, based on the fields
/// of this message type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestDynamicExtensions<'a> {
    #[femtopb(fixed32, optional, tag = 2000)]
    pub scalar_extension: ::core::option::Option<u32>,
    #[femtopb(enumeration, optional, tag = 2001)]
    pub enum_extension: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<ForeignEnum>,
    >,
    #[femtopb(enumeration, optional, tag = 2002)]
    pub dynamic_enum_extension: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<test_dynamic_extensions::DynamicEnumType>,
    >,
    #[femtopb(message, optional, tag = 2003)]
    pub message_extension: ::core::option::Option<ForeignMessage<'a>>,
    #[femtopb(message, optional, tag = 2004)]
    pub dynamic_message_extension: ::core::option::Option<
        test_dynamic_extensions::DynamicMessageType<'a>,
    >,
    #[femtopb(string, repeated, tag = 2005)]
    pub repeated_extension: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(sint32, packed, tag = 2006)]
    pub packed_extension: ::femtopb::packed::Packed<
        'a,
        i32,
        ::femtopb::item_encoding::SInt32,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestDynamicExtensions`.
pub mod test_dynamic_extensions {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct DynamicMessageType<'a> {
        #[femtopb(int32, optional, tag = 2100)]
        pub dynamic_field: ::core::option::Option<i32>,
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
    pub enum DynamicEnumType {
        #[default]
        DynamicFoo = 2200,
        DynamicBar = 2201,
        DynamicBaz = 2202,
    }
    impl DynamicEnumType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DynamicEnumType::DynamicFoo => "DYNAMIC_FOO",
                DynamicEnumType::DynamicBar => "DYNAMIC_BAR",
                DynamicEnumType::DynamicBaz => "DYNAMIC_BAZ",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DYNAMIC_FOO" => Some(Self::DynamicFoo),
                "DYNAMIC_BAR" => Some(Self::DynamicBar),
                "DYNAMIC_BAZ" => Some(Self::DynamicBaz),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestRepeatedString<'a> {
    #[femtopb(string, repeated, tag = 1)]
    pub repeated_string1: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 2)]
    pub repeated_string2: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(bytes, repeated, tag = 11)]
    pub repeated_bytes11: ::femtopb::repeated::Repeated<
        'a,
        &'a [u8],
        ::femtopb::item_encoding::Bytes,
    >,
    #[femtopb(bytes, repeated, tag = 12)]
    pub repeated_bytes12: ::femtopb::repeated::Repeated<
        'a,
        &'a [u8],
        ::femtopb::item_encoding::Bytes,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestRepeatedScalarDifferentTagSizes<'a> {
    /// Parsing repeated fixed size values used to fail. This message needs to be
    /// used in order to get a tag of the right size; all of the repeated fields
    /// in TestAllTypes didn't trigger the check.
    #[femtopb(fixed32, repeated, tag = 12)]
    pub repeated_fixed32: ::femtopb::repeated::Repeated<
        'a,
        u32,
        ::femtopb::item_encoding::Fixed32,
    >,
    /// Check for a varint type, just for good measure.
    #[femtopb(int32, repeated, tag = 13)]
    pub repeated_int32: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    /// These have two-byte tags.
    #[femtopb(fixed64, repeated, tag = 2046)]
    pub repeated_fixed64: ::femtopb::repeated::Repeated<
        'a,
        u64,
        ::femtopb::item_encoding::Fixed64,
    >,
    #[femtopb(int64, repeated, tag = 2047)]
    pub repeated_int64: ::femtopb::repeated::Repeated<
        'a,
        i64,
        ::femtopb::item_encoding::Int64,
    >,
    /// Three byte tags.
    #[femtopb(float, repeated, tag = 262142)]
    pub repeated_float: ::femtopb::repeated::Repeated<
        'a,
        f32,
        ::femtopb::item_encoding::Float,
    >,
    #[femtopb(uint64, repeated, tag = 262143)]
    pub repeated_uint64: ::femtopb::repeated::Repeated<
        'a,
        u64,
        ::femtopb::item_encoding::UInt64,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Test that if an optional or required message/group field appears multiple
/// times in the input, they need to be merged.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestParsingMerge<'a> {
    #[femtopb(message, required, tag = 1)]
    pub required_all_types: TestAllTypes<'a>,
    #[femtopb(message, optional, tag = 2)]
    pub optional_all_types: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(message, repeated, tag = 3)]
    pub repeated_all_types: ::femtopb::repeated::Repeated<
        'a,
        TestAllTypes<'a>,
        ::femtopb::item_encoding::Message<'a, TestAllTypes<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestParsingMerge`.
pub mod test_parsing_merge {
    /// RepeatedFieldsGenerator defines matching field types as TestParsingMerge,
    /// except that all fields are repeated. In the tests, we will serialize the
    /// RepeatedFieldsGenerator to bytes, and parse the bytes to TestParsingMerge.
    /// Repeated fields in RepeatedFieldsGenerator are expected to be merged into
    /// the corresponding required/optional fields in TestParsingMerge.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct RepeatedFieldsGenerator<'a> {
        #[femtopb(message, repeated, tag = 1)]
        pub field1: ::femtopb::repeated::Repeated<
            'a,
            super::TestAllTypes<'a>,
            ::femtopb::item_encoding::Message<'a, super::TestAllTypes<'a>>,
        >,
        #[femtopb(message, repeated, tag = 2)]
        pub field2: ::femtopb::repeated::Repeated<
            'a,
            super::TestAllTypes<'a>,
            ::femtopb::item_encoding::Message<'a, super::TestAllTypes<'a>>,
        >,
        #[femtopb(message, repeated, tag = 3)]
        pub field3: ::femtopb::repeated::Repeated<
            'a,
            super::TestAllTypes<'a>,
            ::femtopb::item_encoding::Message<'a, super::TestAllTypes<'a>>,
        >,
        ///     repeated group Group1 = 10 {
        ///       optional TestAllTypes field1 = 11;
        ///     }
        ///     repeated group Group2 = 20 {
        ///       optional TestAllTypes field1 = 21;
        ///     }
        #[femtopb(message, repeated, tag = 1000)]
        pub ext1: ::femtopb::repeated::Repeated<
            'a,
            super::TestAllTypes<'a>,
            ::femtopb::item_encoding::Message<'a, super::TestAllTypes<'a>>,
        >,
        #[femtopb(message, repeated, tag = 1001)]
        pub ext2: ::femtopb::repeated::Repeated<
            'a,
            super::TestAllTypes<'a>,
            ::femtopb::item_encoding::Message<'a, super::TestAllTypes<'a>>,
        >,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
}
/// Test that the correct exception is thrown by parseFrom in a corner case
/// involving merging, extensions, and required fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestMergeException<'a> {
    #[femtopb(message, optional, tag = 1)]
    pub all_extensions: ::core::option::Option<TestAllExtensions<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestCommentInjectionMessage<'a> {
    /// */ <- This should not close the generated doc comment
    #[femtopb(string, optional, tag = 1, default = "*/ <- Neither should this.")]
    pub a: ::core::option::Option<&'a str>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Used to check that the c++ code generator re-orders messages to reduce
/// padding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestMessageSize<'a> {
    #[femtopb(bool, optional, tag = 1)]
    pub m1: ::core::option::Option<bool>,
    #[femtopb(int64, optional, tag = 2)]
    pub m2: ::core::option::Option<i64>,
    #[femtopb(bool, optional, tag = 3)]
    pub m3: ::core::option::Option<bool>,
    #[femtopb(string, optional, tag = 4)]
    pub m4: ::core::option::Option<&'a str>,
    #[femtopb(int32, optional, tag = 5)]
    pub m5: ::core::option::Option<i32>,
    #[femtopb(int64, optional, tag = 6)]
    pub m6: ::core::option::Option<i64>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Test that RPC services work.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct FooRequest<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct FooResponse<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct FooClientMessage<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct FooServerMessage<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BarRequest<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BarResponse<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestJsonName<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub field_name1: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 2)]
    pub field_name2: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 3)]
    pub field_name3: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 4)]
    pub field_name4: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 5)]
    pub field_name5: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 6)]
    pub field_name6: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 7)]
    pub fieldname7: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestHugeFieldNumbers<'a> {
    #[femtopb(int32, optional, tag = 536870000)]
    pub optional_int32: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 536870001)]
    pub fixed_32: ::core::option::Option<i32>,
    #[femtopb(int32, repeated, tag = 536870002)]
    pub repeated_int32: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    #[femtopb(int32, packed, tag = 536870003)]
    pub packed_int32: ::femtopb::packed::Packed<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    #[femtopb(enumeration, optional, tag = 536870004)]
    pub optional_enum: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<ForeignEnum>,
    >,
    #[femtopb(string, optional, tag = 536870005)]
    pub optional_string: ::core::option::Option<&'a str>,
    #[femtopb(bytes, optional, tag = 536870006)]
    pub optional_bytes: ::core::option::Option<&'a [u8]>,
    #[femtopb(message, optional, tag = 536870007)]
    pub optional_message: ::core::option::Option<ForeignMessage<'a>>,
    #[femtopb(oneof, tags = [536870011, 536870012, 536870013, 536870014])]
    pub oneof_field: ::core::option::Option<test_huge_field_numbers::OneofField<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestHugeFieldNumbers`.
pub mod test_huge_field_numbers {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum OneofField<'a> {
        #[femtopb(uint32, tag = 536870011)]
        OneofUint32(u32),
        #[femtopb(message, tag = 536870012)]
        OneofTestAllTypes(super::TestAllTypes<'a>),
        #[femtopb(string, tag = 536870013)]
        OneofString(&'a str),
        #[femtopb(bytes, tag = 536870014)]
        OneofBytes(&'a [u8]),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestExtensionInsideTable<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub field1: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 2)]
    pub field2: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 3)]
    pub field3: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 4)]
    pub field4: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 6)]
    pub field6: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 7)]
    pub field7: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 8)]
    pub field8: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 9)]
    pub field9: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 10)]
    pub field10: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// NOTE: Intentionally nested to mirror go/glep.
///
///   optional group Layer1OptionalGroup = 1 {
///     repeated group Layer2RepeatedGroup = 2 {
///       extensions 3
///         // NOTE: extension metadata is not supported due to targets such as
///         // `//third_party/protobuf_legacy_opensource/src:shell_scripts_test`,
///         // eee <https://screenshot.googleplex.com/Axz2QD8nxjdpyFF>
///         //[metadata = {
///         // NOTE: can't write type there due to some clever build gen code at
///         // <http://google3/net/proto2/internal/BUILD;l=1247;rcl=411090862>
///         // type: "protobuf_unittest.TestNestedGroupExtensionInnerExtension",
///         // name: "inner",
///         // }]
///       ;
///       optional string another_field = 6;
///     }
///     repeated group Layer2AnotherOptionalRepeatedGroup = 4 {
///       optional string but_why_tho = 5;
///     }
///   }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestNestedGroupExtensionOuter<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestNestedGroupExtensionInnerExtension<'a> {
    #[femtopb(string, optional, tag = 1)]
    pub inner_name: ::core::option::Option<&'a str>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestExtensionRangeSerialize<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub foo_one: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 6)]
    pub foo_two: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 7)]
    pub foo_three: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 13)]
    pub foo_four: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestVerifyInt32Simple<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub optional_int32_1: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 2)]
    pub optional_int32_2: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 63)]
    pub optional_int32_63: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 64)]
    pub optional_int32_64: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestVerifyInt32<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub optional_int32_1: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 2)]
    pub optional_int32_2: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 63)]
    pub optional_int32_63: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 64)]
    pub optional_int32_64: ::core::option::Option<i32>,
    #[femtopb(message, optional, tag = 9)]
    pub optional_all_types: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(message, repeated, tag = 10)]
    pub repeated_all_types: ::femtopb::repeated::Repeated<
        'a,
        TestAllTypes<'a>,
        ::femtopb::item_encoding::Message<'a, TestAllTypes<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestVerifyMostlyInt32<'a> {
    #[femtopb(int64, optional, tag = 30)]
    pub optional_int64_30: ::core::option::Option<i64>,
    #[femtopb(int32, optional, tag = 1)]
    pub optional_int32_1: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 2)]
    pub optional_int32_2: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 3)]
    pub optional_int32_3: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 4)]
    pub optional_int32_4: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 63)]
    pub optional_int32_63: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 64)]
    pub optional_int32_64: ::core::option::Option<i32>,
    #[femtopb(message, optional, tag = 9)]
    pub optional_all_types: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(message, repeated, tag = 10)]
    pub repeated_all_types: ::femtopb::repeated::Repeated<
        'a,
        TestAllTypes<'a>,
        ::femtopb::item_encoding::Message<'a, TestAllTypes<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestVerifyMostlyInt32BigFieldNumber<'a> {
    #[femtopb(int64, optional, tag = 30)]
    pub optional_int64_30: ::core::option::Option<i64>,
    #[femtopb(int32, optional, tag = 300)]
    pub optional_int32_300: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 1)]
    pub optional_int32_1: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 2)]
    pub optional_int32_2: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 3)]
    pub optional_int32_3: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 4)]
    pub optional_int32_4: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 63)]
    pub optional_int32_63: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 64)]
    pub optional_int32_64: ::core::option::Option<i32>,
    #[femtopb(message, optional, tag = 9)]
    pub optional_all_types: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(message, repeated, tag = 10)]
    pub repeated_all_types: ::femtopb::repeated::Repeated<
        'a,
        TestAllTypes<'a>,
        ::femtopb::item_encoding::Message<'a, TestAllTypes<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestVerifyUint32Simple<'a> {
    #[femtopb(uint32, optional, tag = 1)]
    pub optional_uint32_1: ::core::option::Option<u32>,
    #[femtopb(uint32, optional, tag = 2)]
    pub optional_uint32_2: ::core::option::Option<u32>,
    #[femtopb(uint32, optional, tag = 63)]
    pub optional_uint32_63: ::core::option::Option<u32>,
    #[femtopb(uint32, optional, tag = 64)]
    pub optional_uint32_64: ::core::option::Option<u32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestVerifyUint32<'a> {
    #[femtopb(uint32, optional, tag = 1)]
    pub optional_uint32_1: ::core::option::Option<u32>,
    #[femtopb(uint32, optional, tag = 2)]
    pub optional_uint32_2: ::core::option::Option<u32>,
    #[femtopb(uint32, optional, tag = 63)]
    pub optional_uint32_63: ::core::option::Option<u32>,
    #[femtopb(uint32, optional, tag = 64)]
    pub optional_uint32_64: ::core::option::Option<u32>,
    #[femtopb(message, optional, tag = 9)]
    pub optional_all_types: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(message, repeated, tag = 10)]
    pub repeated_all_types: ::femtopb::repeated::Repeated<
        'a,
        TestAllTypes<'a>,
        ::femtopb::item_encoding::Message<'a, TestAllTypes<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestVerifyOneUint32<'a> {
    #[femtopb(uint32, optional, tag = 1)]
    pub optional_uint32_1: ::core::option::Option<u32>,
    #[femtopb(int32, optional, tag = 2)]
    pub optional_int32_2: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 63)]
    pub optional_int32_63: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 64)]
    pub optional_int32_64: ::core::option::Option<i32>,
    #[femtopb(message, optional, tag = 9)]
    pub optional_all_types: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(message, repeated, tag = 10)]
    pub repeated_all_types: ::femtopb::repeated::Repeated<
        'a,
        TestAllTypes<'a>,
        ::femtopb::item_encoding::Message<'a, TestAllTypes<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestVerifyOneInt32BigFieldNumber<'a> {
    #[femtopb(int32, optional, tag = 65)]
    pub optional_int32_65: ::core::option::Option<i32>,
    #[femtopb(int64, optional, tag = 1)]
    pub optional_int64_1: ::core::option::Option<i64>,
    #[femtopb(int64, optional, tag = 2)]
    pub optional_int64_2: ::core::option::Option<i64>,
    #[femtopb(int64, optional, tag = 63)]
    pub optional_int64_63: ::core::option::Option<i64>,
    #[femtopb(int64, optional, tag = 64)]
    pub optional_int64_64: ::core::option::Option<i64>,
    #[femtopb(message, optional, tag = 9)]
    pub optional_all_types: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(message, repeated, tag = 10)]
    pub repeated_all_types: ::femtopb::repeated::Repeated<
        'a,
        TestAllTypes<'a>,
        ::femtopb::item_encoding::Message<'a, TestAllTypes<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestVerifyInt32BigFieldNumber<'a> {
    #[femtopb(int32, optional, tag = 1000)]
    pub optional_int32_1000: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 65)]
    pub optional_int32_65: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 1)]
    pub optional_int32_1: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 2)]
    pub optional_int32_2: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 63)]
    pub optional_int32_63: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 64)]
    pub optional_int32_64: ::core::option::Option<i32>,
    #[femtopb(message, optional, tag = 9)]
    pub optional_all_types: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(message, repeated, tag = 10)]
    pub repeated_all_types: ::femtopb::repeated::Repeated<
        'a,
        TestAllTypes<'a>,
        ::femtopb::item_encoding::Message<'a, TestAllTypes<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestVerifyUint32BigFieldNumber<'a> {
    #[femtopb(uint32, optional, tag = 1000)]
    pub optional_uint32_1000: ::core::option::Option<u32>,
    #[femtopb(uint32, optional, tag = 65)]
    pub optional_uint32_65: ::core::option::Option<u32>,
    #[femtopb(uint32, optional, tag = 1)]
    pub optional_uint32_1: ::core::option::Option<u32>,
    #[femtopb(uint32, optional, tag = 2)]
    pub optional_uint32_2: ::core::option::Option<u32>,
    #[femtopb(uint32, optional, tag = 63)]
    pub optional_uint32_63: ::core::option::Option<u32>,
    #[femtopb(uint32, optional, tag = 64)]
    pub optional_uint32_64: ::core::option::Option<u32>,
    #[femtopb(message, optional, tag = 9)]
    pub optional_all_types: ::core::option::Option<TestAllTypes<'a>>,
    #[femtopb(message, repeated, tag = 10)]
    pub repeated_all_types: ::femtopb::repeated::Repeated<
        'a,
        TestAllTypes<'a>,
        ::femtopb::item_encoding::Message<'a, TestAllTypes<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestVerifyBigFieldNumberUint32<'a> {
    #[femtopb(message, optional, tag = 1)]
    pub optional_nested: ::core::option::Option<
        test_verify_big_field_number_uint32::Nested<'a>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestVerifyBigFieldNumberUint32`.
pub mod test_verify_big_field_number_uint32 {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct Nested<'a> {
        #[femtopb(uint32, optional, tag = 5000)]
        pub optional_uint32_5000: ::core::option::Option<u32>,
        #[femtopb(uint32, optional, tag = 1000)]
        pub optional_uint32_1000: ::core::option::Option<u32>,
        #[femtopb(uint32, optional, tag = 66)]
        pub optional_uint32_66: ::core::option::Option<u32>,
        #[femtopb(uint32, optional, tag = 65)]
        pub optional_uint32_65: ::core::option::Option<u32>,
        #[femtopb(uint32, optional, tag = 1)]
        pub optional_uint32_1: ::core::option::Option<u32>,
        #[femtopb(uint32, optional, tag = 2)]
        pub optional_uint32_2: ::core::option::Option<u32>,
        #[femtopb(uint32, optional, tag = 63)]
        pub optional_uint32_63: ::core::option::Option<u32>,
        #[femtopb(uint32, optional, tag = 64)]
        pub optional_uint32_64: ::core::option::Option<u32>,
        #[femtopb(message, optional, deferred, tag = 9)]
        pub optional_nested: ::core::option::Option<
            ::femtopb::deferred::Deferred<'a, Nested<'a>>,
        >,
        #[femtopb(message, repeated, tag = 10)]
        pub repeated_nested: ::femtopb::repeated::Repeated<
            'a,
            Nested<'a>,
            ::femtopb::item_encoding::Message<'a, Nested<'a>>,
        >,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
}
/// This message contains different kind of enums to exercise the different
/// parsers in table-driven.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct EnumParseTester<'a> {
    #[femtopb(enumeration, optional, tag = 1)]
    pub optional_seq_small_0_lowfield: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall0>,
    >,
    #[femtopb(enumeration, optional, tag = 1001)]
    pub optional_seq_small_0_midfield: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall0>,
    >,
    #[femtopb(enumeration, optional, tag = 1000001)]
    pub optional_seq_small_0_hifield: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall0>,
    >,
    #[femtopb(enumeration, repeated, tag = 2)]
    pub repeated_seq_small_0_lowfield: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall0>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqSmall0>,
    >,
    #[femtopb(enumeration, repeated, tag = 1002)]
    pub repeated_seq_small_0_midfield: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall0>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqSmall0>,
    >,
    #[femtopb(enumeration, repeated, tag = 1000002)]
    pub repeated_seq_small_0_hifield: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall0>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqSmall0>,
    >,
    #[femtopb(enumeration, packed, tag = 3)]
    pub packed_seq_small_0_lowfield: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall0>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqSmall0>,
    >,
    #[femtopb(enumeration, packed, tag = 1003)]
    pub packed_seq_small_0_midfield: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall0>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqSmall0>,
    >,
    #[femtopb(enumeration, packed, tag = 1000003)]
    pub packed_seq_small_0_hifield: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall0>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqSmall0>,
    >,
    #[femtopb(enumeration, optional, tag = 4)]
    pub optional_seq_small_1_lowfield: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall1>,
    >,
    #[femtopb(enumeration, optional, tag = 1004)]
    pub optional_seq_small_1_midfield: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall1>,
    >,
    #[femtopb(enumeration, optional, tag = 1000004)]
    pub optional_seq_small_1_hifield: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall1>,
    >,
    #[femtopb(enumeration, repeated, tag = 5)]
    pub repeated_seq_small_1_lowfield: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall1>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqSmall1>,
    >,
    #[femtopb(enumeration, repeated, tag = 1005)]
    pub repeated_seq_small_1_midfield: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall1>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqSmall1>,
    >,
    #[femtopb(enumeration, repeated, tag = 1000005)]
    pub repeated_seq_small_1_hifield: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall1>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqSmall1>,
    >,
    #[femtopb(enumeration, packed, tag = 6)]
    pub packed_seq_small_1_lowfield: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall1>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqSmall1>,
    >,
    #[femtopb(enumeration, packed, tag = 1006)]
    pub packed_seq_small_1_midfield: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall1>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqSmall1>,
    >,
    #[femtopb(enumeration, packed, tag = 1000006)]
    pub packed_seq_small_1_hifield: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqSmall1>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqSmall1>,
    >,
    #[femtopb(enumeration, optional, tag = 7)]
    pub optional_seq_large_lowfield: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqLarge>,
    >,
    #[femtopb(enumeration, optional, tag = 1007)]
    pub optional_seq_large_midfield: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqLarge>,
    >,
    #[femtopb(enumeration, optional, tag = 1000007)]
    pub optional_seq_large_hifield: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqLarge>,
    >,
    #[femtopb(enumeration, repeated, tag = 8)]
    pub repeated_seq_large_lowfield: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqLarge>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqLarge>,
    >,
    #[femtopb(enumeration, repeated, tag = 1008)]
    pub repeated_seq_large_midfield: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqLarge>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqLarge>,
    >,
    #[femtopb(enumeration, repeated, tag = 1000008)]
    pub repeated_seq_large_hifield: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqLarge>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqLarge>,
    >,
    #[femtopb(enumeration, packed, tag = 9)]
    pub packed_seq_large_lowfield: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqLarge>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqLarge>,
    >,
    #[femtopb(enumeration, packed, tag = 1009)]
    pub packed_seq_large_midfield: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqLarge>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqLarge>,
    >,
    #[femtopb(enumeration, packed, tag = 1000009)]
    pub packed_seq_large_hifield: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::SeqLarge>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::SeqLarge>,
    >,
    #[femtopb(enumeration, optional, tag = 10)]
    pub optional_arbitrary_lowfield: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<enum_parse_tester::Arbitrary>,
    >,
    #[femtopb(enumeration, optional, tag = 1010)]
    pub optional_arbitrary_midfield: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<enum_parse_tester::Arbitrary>,
    >,
    #[femtopb(enumeration, optional, tag = 1000010)]
    pub optional_arbitrary_hifield: ::core::option::Option<
        ::femtopb::enumeration::EnumValue<enum_parse_tester::Arbitrary>,
    >,
    #[femtopb(enumeration, repeated, tag = 11)]
    pub repeated_arbitrary_lowfield: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::Arbitrary>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::Arbitrary>,
    >,
    #[femtopb(enumeration, repeated, tag = 1011)]
    pub repeated_arbitrary_midfield: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::Arbitrary>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::Arbitrary>,
    >,
    #[femtopb(enumeration, repeated, tag = 1000011)]
    pub repeated_arbitrary_hifield: ::femtopb::repeated::Repeated<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::Arbitrary>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::Arbitrary>,
    >,
    #[femtopb(enumeration, packed, tag = 12)]
    pub packed_arbitrary_lowfield: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::Arbitrary>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::Arbitrary>,
    >,
    #[femtopb(enumeration, packed, tag = 1012)]
    pub packed_arbitrary_midfield: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::Arbitrary>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::Arbitrary>,
    >,
    #[femtopb(enumeration, packed, tag = 1000012)]
    pub packed_arbitrary_hifield: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<enum_parse_tester::Arbitrary>,
        ::femtopb::item_encoding::Enum<enum_parse_tester::Arbitrary>,
    >,
    /// An arbitrary field we can append to to break the runs of repeated fields.
    #[femtopb(int32, optional, tag = 99)]
    pub other_field: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `EnumParseTester`.
pub mod enum_parse_tester {
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
    pub enum SeqSmall0 {
        #[default]
        Default = 0,
        SeqSmall01 = 1,
        SeqSmall02 = 2,
    }
    impl SeqSmall0 {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SeqSmall0::Default => "SEQ_SMALL_0_DEFAULT",
                SeqSmall0::SeqSmall01 => "SEQ_SMALL_0_1",
                SeqSmall0::SeqSmall02 => "SEQ_SMALL_0_2",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEQ_SMALL_0_DEFAULT" => Some(Self::Default),
                "SEQ_SMALL_0_1" => Some(Self::SeqSmall01),
                "SEQ_SMALL_0_2" => Some(Self::SeqSmall02),
                _ => None,
            }
        }
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
    pub enum SeqSmall1 {
        #[default]
        Default = 1,
        SeqSmall12 = 2,
        SeqSmall13 = 3,
    }
    impl SeqSmall1 {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SeqSmall1::Default => "SEQ_SMALL_1_DEFAULT",
                SeqSmall1::SeqSmall12 => "SEQ_SMALL_1_2",
                SeqSmall1::SeqSmall13 => "SEQ_SMALL_1_3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEQ_SMALL_1_DEFAULT" => Some(Self::Default),
                "SEQ_SMALL_1_2" => Some(Self::SeqSmall12),
                "SEQ_SMALL_1_3" => Some(Self::SeqSmall13),
                _ => None,
            }
        }
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
    pub enum SeqLarge {
        #[default]
        Default = -1,
        SeqLarge0 = 0,
        SeqLarge1 = 1,
        SeqLarge2 = 2,
        SeqLarge3 = 3,
        SeqLarge4 = 4,
        SeqLarge5 = 5,
        SeqLarge6 = 6,
        SeqLarge7 = 7,
        SeqLarge8 = 8,
        SeqLarge9 = 9,
        SeqLarge10 = 10,
        SeqLarge11 = 11,
        SeqLarge12 = 12,
        SeqLarge13 = 13,
        SeqLarge14 = 14,
        SeqLarge15 = 15,
        SeqLarge16 = 16,
        SeqLarge17 = 17,
        SeqLarge18 = 18,
        SeqLarge19 = 19,
        SeqLarge20 = 20,
        SeqLarge21 = 21,
        SeqLarge22 = 22,
        SeqLarge23 = 23,
        SeqLarge24 = 24,
        SeqLarge25 = 25,
        SeqLarge26 = 26,
        SeqLarge27 = 27,
        SeqLarge28 = 28,
        SeqLarge29 = 29,
        SeqLarge30 = 30,
        SeqLarge31 = 31,
        SeqLarge32 = 32,
        SeqLarge33 = 33,
    }
    impl SeqLarge {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SeqLarge::Default => "SEQ_LARGE_DEFAULT",
                SeqLarge::SeqLarge0 => "SEQ_LARGE_0",
                SeqLarge::SeqLarge1 => "SEQ_LARGE_1",
                SeqLarge::SeqLarge2 => "SEQ_LARGE_2",
                SeqLarge::SeqLarge3 => "SEQ_LARGE_3",
                SeqLarge::SeqLarge4 => "SEQ_LARGE_4",
                SeqLarge::SeqLarge5 => "SEQ_LARGE_5",
                SeqLarge::SeqLarge6 => "SEQ_LARGE_6",
                SeqLarge::SeqLarge7 => "SEQ_LARGE_7",
                SeqLarge::SeqLarge8 => "SEQ_LARGE_8",
                SeqLarge::SeqLarge9 => "SEQ_LARGE_9",
                SeqLarge::SeqLarge10 => "SEQ_LARGE_10",
                SeqLarge::SeqLarge11 => "SEQ_LARGE_11",
                SeqLarge::SeqLarge12 => "SEQ_LARGE_12",
                SeqLarge::SeqLarge13 => "SEQ_LARGE_13",
                SeqLarge::SeqLarge14 => "SEQ_LARGE_14",
                SeqLarge::SeqLarge15 => "SEQ_LARGE_15",
                SeqLarge::SeqLarge16 => "SEQ_LARGE_16",
                SeqLarge::SeqLarge17 => "SEQ_LARGE_17",
                SeqLarge::SeqLarge18 => "SEQ_LARGE_18",
                SeqLarge::SeqLarge19 => "SEQ_LARGE_19",
                SeqLarge::SeqLarge20 => "SEQ_LARGE_20",
                SeqLarge::SeqLarge21 => "SEQ_LARGE_21",
                SeqLarge::SeqLarge22 => "SEQ_LARGE_22",
                SeqLarge::SeqLarge23 => "SEQ_LARGE_23",
                SeqLarge::SeqLarge24 => "SEQ_LARGE_24",
                SeqLarge::SeqLarge25 => "SEQ_LARGE_25",
                SeqLarge::SeqLarge26 => "SEQ_LARGE_26",
                SeqLarge::SeqLarge27 => "SEQ_LARGE_27",
                SeqLarge::SeqLarge28 => "SEQ_LARGE_28",
                SeqLarge::SeqLarge29 => "SEQ_LARGE_29",
                SeqLarge::SeqLarge30 => "SEQ_LARGE_30",
                SeqLarge::SeqLarge31 => "SEQ_LARGE_31",
                SeqLarge::SeqLarge32 => "SEQ_LARGE_32",
                SeqLarge::SeqLarge33 => "SEQ_LARGE_33",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEQ_LARGE_DEFAULT" => Some(Self::Default),
                "SEQ_LARGE_0" => Some(Self::SeqLarge0),
                "SEQ_LARGE_1" => Some(Self::SeqLarge1),
                "SEQ_LARGE_2" => Some(Self::SeqLarge2),
                "SEQ_LARGE_3" => Some(Self::SeqLarge3),
                "SEQ_LARGE_4" => Some(Self::SeqLarge4),
                "SEQ_LARGE_5" => Some(Self::SeqLarge5),
                "SEQ_LARGE_6" => Some(Self::SeqLarge6),
                "SEQ_LARGE_7" => Some(Self::SeqLarge7),
                "SEQ_LARGE_8" => Some(Self::SeqLarge8),
                "SEQ_LARGE_9" => Some(Self::SeqLarge9),
                "SEQ_LARGE_10" => Some(Self::SeqLarge10),
                "SEQ_LARGE_11" => Some(Self::SeqLarge11),
                "SEQ_LARGE_12" => Some(Self::SeqLarge12),
                "SEQ_LARGE_13" => Some(Self::SeqLarge13),
                "SEQ_LARGE_14" => Some(Self::SeqLarge14),
                "SEQ_LARGE_15" => Some(Self::SeqLarge15),
                "SEQ_LARGE_16" => Some(Self::SeqLarge16),
                "SEQ_LARGE_17" => Some(Self::SeqLarge17),
                "SEQ_LARGE_18" => Some(Self::SeqLarge18),
                "SEQ_LARGE_19" => Some(Self::SeqLarge19),
                "SEQ_LARGE_20" => Some(Self::SeqLarge20),
                "SEQ_LARGE_21" => Some(Self::SeqLarge21),
                "SEQ_LARGE_22" => Some(Self::SeqLarge22),
                "SEQ_LARGE_23" => Some(Self::SeqLarge23),
                "SEQ_LARGE_24" => Some(Self::SeqLarge24),
                "SEQ_LARGE_25" => Some(Self::SeqLarge25),
                "SEQ_LARGE_26" => Some(Self::SeqLarge26),
                "SEQ_LARGE_27" => Some(Self::SeqLarge27),
                "SEQ_LARGE_28" => Some(Self::SeqLarge28),
                "SEQ_LARGE_29" => Some(Self::SeqLarge29),
                "SEQ_LARGE_30" => Some(Self::SeqLarge30),
                "SEQ_LARGE_31" => Some(Self::SeqLarge31),
                "SEQ_LARGE_32" => Some(Self::SeqLarge32),
                "SEQ_LARGE_33" => Some(Self::SeqLarge33),
                _ => None,
            }
        }
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
    pub enum Arbitrary {
        #[default]
        Default = -123123,
        Arbitrary1 = -123,
        Arbitrary2 = 213,
        Arbitrary3 = 213213,
        Min = -2147483648,
        Max = 2147483647,
    }
    impl Arbitrary {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Arbitrary::Default => "ARBITRARY_DEFAULT",
                Arbitrary::Arbitrary1 => "ARBITRARY_1",
                Arbitrary::Arbitrary2 => "ARBITRARY_2",
                Arbitrary::Arbitrary3 => "ARBITRARY_3",
                Arbitrary::Min => "ARBITRARY_MIN",
                Arbitrary::Max => "ARBITRARY_MAX",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ARBITRARY_DEFAULT" => Some(Self::Default),
                "ARBITRARY_1" => Some(Self::Arbitrary1),
                "ARBITRARY_2" => Some(Self::Arbitrary2),
                "ARBITRARY_3" => Some(Self::Arbitrary3),
                "ARBITRARY_MIN" => Some(Self::Min),
                "ARBITRARY_MAX" => Some(Self::Max),
                _ => None,
            }
        }
    }
}
/// This message contains different kind of bool fields to exercise the different
/// parsers in table-drived.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BoolParseTester<'a> {
    #[femtopb(bool, optional, tag = 1)]
    pub optional_bool_lowfield: ::core::option::Option<bool>,
    #[femtopb(bool, optional, tag = 1001)]
    pub optional_bool_midfield: ::core::option::Option<bool>,
    #[femtopb(bool, optional, tag = 1000001)]
    pub optional_bool_hifield: ::core::option::Option<bool>,
    #[femtopb(bool, repeated, tag = 2)]
    pub repeated_bool_lowfield: ::femtopb::repeated::Repeated<
        'a,
        bool,
        ::femtopb::item_encoding::Bool,
    >,
    #[femtopb(bool, repeated, tag = 1002)]
    pub repeated_bool_midfield: ::femtopb::repeated::Repeated<
        'a,
        bool,
        ::femtopb::item_encoding::Bool,
    >,
    #[femtopb(bool, repeated, tag = 1000002)]
    pub repeated_bool_hifield: ::femtopb::repeated::Repeated<
        'a,
        bool,
        ::femtopb::item_encoding::Bool,
    >,
    #[femtopb(bool, packed, tag = 3)]
    pub packed_bool_lowfield: ::femtopb::packed::Packed<
        'a,
        bool,
        ::femtopb::item_encoding::Bool,
    >,
    #[femtopb(bool, packed, tag = 1003)]
    pub packed_bool_midfield: ::femtopb::packed::Packed<
        'a,
        bool,
        ::femtopb::item_encoding::Bool,
    >,
    #[femtopb(bool, packed, tag = 1000003)]
    pub packed_bool_hifield: ::femtopb::packed::Packed<
        'a,
        bool,
        ::femtopb::item_encoding::Bool,
    >,
    /// An arbitrary field we can append to to break the runs of repeated fields.
    #[femtopb(int32, optional, tag = 99)]
    pub other_field: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Int32ParseTester<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub optional_int32_lowfield: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 1001)]
    pub optional_int32_midfield: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 1000001)]
    pub optional_int32_hifield: ::core::option::Option<i32>,
    #[femtopb(int32, repeated, tag = 2)]
    pub repeated_int32_lowfield: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    #[femtopb(int32, repeated, tag = 1002)]
    pub repeated_int32_midfield: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    #[femtopb(int32, repeated, tag = 1000002)]
    pub repeated_int32_hifield: ::femtopb::repeated::Repeated<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    #[femtopb(int32, packed, tag = 3)]
    pub packed_int32_lowfield: ::femtopb::packed::Packed<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    #[femtopb(int32, packed, tag = 1003)]
    pub packed_int32_midfield: ::femtopb::packed::Packed<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    #[femtopb(int32, packed, tag = 1000003)]
    pub packed_int32_hifield: ::femtopb::packed::Packed<
        'a,
        i32,
        ::femtopb::item_encoding::Int32,
    >,
    /// An arbitrary field we can append to to break the runs of repeated fields.
    #[femtopb(int32, optional, tag = 99)]
    pub other_field: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Int64ParseTester<'a> {
    #[femtopb(int64, optional, tag = 1)]
    pub optional_int64_lowfield: ::core::option::Option<i64>,
    #[femtopb(int64, optional, tag = 1001)]
    pub optional_int64_midfield: ::core::option::Option<i64>,
    #[femtopb(int64, optional, tag = 1000001)]
    pub optional_int64_hifield: ::core::option::Option<i64>,
    #[femtopb(int64, repeated, tag = 2)]
    pub repeated_int64_lowfield: ::femtopb::repeated::Repeated<
        'a,
        i64,
        ::femtopb::item_encoding::Int64,
    >,
    #[femtopb(int64, repeated, tag = 1002)]
    pub repeated_int64_midfield: ::femtopb::repeated::Repeated<
        'a,
        i64,
        ::femtopb::item_encoding::Int64,
    >,
    #[femtopb(int64, repeated, tag = 1000002)]
    pub repeated_int64_hifield: ::femtopb::repeated::Repeated<
        'a,
        i64,
        ::femtopb::item_encoding::Int64,
    >,
    #[femtopb(int64, packed, tag = 3)]
    pub packed_int64_lowfield: ::femtopb::packed::Packed<
        'a,
        i64,
        ::femtopb::item_encoding::Int64,
    >,
    #[femtopb(int64, packed, tag = 1003)]
    pub packed_int64_midfield: ::femtopb::packed::Packed<
        'a,
        i64,
        ::femtopb::item_encoding::Int64,
    >,
    #[femtopb(int64, packed, tag = 1000003)]
    pub packed_int64_hifield: ::femtopb::packed::Packed<
        'a,
        i64,
        ::femtopb::item_encoding::Int64,
    >,
    /// An arbitrary field we can append to to break the runs of repeated fields.
    #[femtopb(int32, optional, tag = 99)]
    pub other_field: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct InlinedStringIdxRegressionProto<'a> {
    /// We mix data to make sure aux ids and inlined string idx do not match.
    /// aux_idx == inlined_string_idx == 1
    #[femtopb(string, optional, tag = 1)]
    pub str1: ::core::option::Option<&'a str>,
    /// aux_idx == 2
    #[femtopb(message, optional, deferred, tag = 2)]
    pub sub: ::core::option::Option<
        ::femtopb::deferred::Deferred<'a, InlinedStringIdxRegressionProto<'a>>,
    >,
    /// aux_idx == 3, inlined_string_idx == 2
    #[femtopb(string, optional, tag = 3)]
    pub str2: ::core::option::Option<&'a str>,
    /// aux_idx == 4, inlined_string_idx == 3
    #[femtopb(bytes, optional, tag = 4)]
    pub str3: ::core::option::Option<&'a [u8]>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct StringParseTester<'a> {
    #[femtopb(string, optional, tag = 1)]
    pub optional_string_lowfield: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 1001)]
    pub optional_string_midfield: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 1000001)]
    pub optional_string_hifield: ::core::option::Option<&'a str>,
    #[femtopb(string, repeated, tag = 2)]
    pub repeated_string_lowfield: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 1002)]
    pub repeated_string_midfield: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 1000002)]
    pub repeated_string_hifield: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct BadFieldNames<'a> {
    #[femtopb(int32, optional, tag = 1)]
    pub optional_int32: ::core::option::Option<i32>,
    #[femtopb(int32, optional, tag = 2)]
    pub r#for: ::core::option::Option<i32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestNestedMessageRedaction<'a> {
    #[femtopb(string, optional, tag = 1)]
    pub optional_unredacted_nested_string: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 2)]
    pub optional_redacted_nested_string: ::core::option::Option<&'a str>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct RedactedFields<'a> {
    #[femtopb(string, optional, tag = 1)]
    pub optional_redacted_string: ::core::option::Option<&'a str>,
    #[femtopb(string, optional, tag = 2)]
    pub optional_unredacted_string: ::core::option::Option<&'a str>,
    #[femtopb(string, repeated, tag = 3)]
    pub repeated_redacted_string: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 4)]
    pub repeated_unredacted_string: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(message, optional, tag = 5)]
    pub optional_redacted_message: ::core::option::Option<
        TestNestedMessageRedaction<'a>,
    >,
    #[femtopb(message, optional, tag = 6)]
    pub optional_unredacted_message: ::core::option::Option<
        TestNestedMessageRedaction<'a>,
    >,
    #[femtopb(message, repeated, tag = 7)]
    pub repeated_redacted_message: ::femtopb::repeated::Repeated<
        'a,
        TestNestedMessageRedaction<'a>,
        ::femtopb::item_encoding::Message<'a, TestNestedMessageRedaction<'a>>,
    >,
    ///   map<string, string> map_redacted_string = 9;
    ///   map<string, string> map_unredacted_string = 10;
    #[femtopb(message, repeated, tag = 8)]
    pub repeated_unredacted_message: ::femtopb::repeated::Repeated<
        'a,
        TestNestedMessageRedaction<'a>,
        ::femtopb::item_encoding::Message<'a, TestNestedMessageRedaction<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestCord<'a> {
    #[femtopb(bytes, optional, tag = 1)]
    pub optional_bytes_cord: ::core::option::Option<&'a [u8]>,
    #[femtopb(bytes, optional, tag = 2, default = b"hello")]
    pub optional_bytes_cord_default: ::core::option::Option<&'a [u8]>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestPackedEnumSmallRange<'a> {
    #[femtopb(enumeration, packed, tag = 1)]
    pub vals: ::femtopb::packed::Packed<
        'a,
        ::femtopb::enumeration::EnumValue<test_packed_enum_small_range::NestedEnum>,
        ::femtopb::item_encoding::Enum<test_packed_enum_small_range::NestedEnum>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TestPackedEnumSmallRange`.
pub mod test_packed_enum_small_range {
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
    pub enum NestedEnum {
        #[default]
        Unspecified = 0,
        Foo = 1,
        Bar = 2,
        Baz = 3,
    }
    impl NestedEnum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NestedEnum::Unspecified => "UNSPECIFIED",
                NestedEnum::Foo => "FOO",
                NestedEnum::Bar => "BAR",
                NestedEnum::Baz => "BAZ",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "FOO" => Some(Self::Foo),
                "BAR" => Some(Self::Bar),
                "BAZ" => Some(Self::Baz),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct EnumsForBenchmark<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `EnumsForBenchmark`.
pub mod enums_for_benchmark {
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
    pub enum Flat {
        #[default]
        A0 = 0,
        A1 = 1,
        A2 = 2,
        A3 = 3,
        A4 = 4,
        A5 = 5,
        A6 = 6,
        A7 = 7,
        A8 = 8,
        A9 = 9,
        A10 = 10,
        A11 = 11,
        A12 = 12,
        A13 = 13,
        A14 = 14,
        A15 = 15,
    }
    impl Flat {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Flat::A0 => "A0",
                Flat::A1 => "A1",
                Flat::A2 => "A2",
                Flat::A3 => "A3",
                Flat::A4 => "A4",
                Flat::A5 => "A5",
                Flat::A6 => "A6",
                Flat::A7 => "A7",
                Flat::A8 => "A8",
                Flat::A9 => "A9",
                Flat::A10 => "A10",
                Flat::A11 => "A11",
                Flat::A12 => "A12",
                Flat::A13 => "A13",
                Flat::A14 => "A14",
                Flat::A15 => "A15",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "A0" => Some(Self::A0),
                "A1" => Some(Self::A1),
                "A2" => Some(Self::A2),
                "A3" => Some(Self::A3),
                "A4" => Some(Self::A4),
                "A5" => Some(Self::A5),
                "A6" => Some(Self::A6),
                "A7" => Some(Self::A7),
                "A8" => Some(Self::A8),
                "A9" => Some(Self::A9),
                "A10" => Some(Self::A10),
                "A11" => Some(Self::A11),
                "A12" => Some(Self::A12),
                "A13" => Some(Self::A13),
                "A14" => Some(Self::A14),
                "A15" => Some(Self::A15),
                _ => None,
            }
        }
    }
    /// Has a few holes, bitmap can be used.
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
    pub enum AlmostFlat {
        #[default]
        B0 = 0,
        B1 = 1,
        B2 = 2,
        B3 = 3,
        B5 = 5,
        B6 = 6,
        B7 = 7,
        B8 = 8,
        B9 = 9,
        B11 = 11,
        B12 = 12,
        B13 = 13,
        B14 = 14,
        B15 = 15,
        B17 = 17,
        B19 = 19,
    }
    impl AlmostFlat {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AlmostFlat::B0 => "B0",
                AlmostFlat::B1 => "B1",
                AlmostFlat::B2 => "B2",
                AlmostFlat::B3 => "B3",
                AlmostFlat::B5 => "B5",
                AlmostFlat::B6 => "B6",
                AlmostFlat::B7 => "B7",
                AlmostFlat::B8 => "B8",
                AlmostFlat::B9 => "B9",
                AlmostFlat::B11 => "B11",
                AlmostFlat::B12 => "B12",
                AlmostFlat::B13 => "B13",
                AlmostFlat::B14 => "B14",
                AlmostFlat::B15 => "B15",
                AlmostFlat::B17 => "B17",
                AlmostFlat::B19 => "B19",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "B0" => Some(Self::B0),
                "B1" => Some(Self::B1),
                "B2" => Some(Self::B2),
                "B3" => Some(Self::B3),
                "B5" => Some(Self::B5),
                "B6" => Some(Self::B6),
                "B7" => Some(Self::B7),
                "B8" => Some(Self::B8),
                "B9" => Some(Self::B9),
                "B11" => Some(Self::B11),
                "B12" => Some(Self::B12),
                "B13" => Some(Self::B13),
                "B14" => Some(Self::B14),
                "B15" => Some(Self::B15),
                "B17" => Some(Self::B17),
                "B19" => Some(Self::B19),
                _ => None,
            }
        }
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
    pub enum Sparse {
        #[default]
        C536 = 536,
        C8387 = 8387,
        C9673 = 9673,
        C10285 = 10285,
        C13318 = 13318,
        C15963 = 15963,
        C16439 = 16439,
        C18197 = 18197,
        C19430 = 19430,
        C20361 = 20361,
        C20706 = 20706,
        C21050 = 21050,
        C21906 = 21906,
        C27265 = 27265,
        C30109 = 30109,
        C31670 = 31670,
    }
    impl Sparse {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Sparse::C536 => "C536",
                Sparse::C8387 => "C8387",
                Sparse::C9673 => "C9673",
                Sparse::C10285 => "C10285",
                Sparse::C13318 => "C13318",
                Sparse::C15963 => "C15963",
                Sparse::C16439 => "C16439",
                Sparse::C18197 => "C18197",
                Sparse::C19430 => "C19430",
                Sparse::C20361 => "C20361",
                Sparse::C20706 => "C20706",
                Sparse::C21050 => "C21050",
                Sparse::C21906 => "C21906",
                Sparse::C27265 => "C27265",
                Sparse::C30109 => "C30109",
                Sparse::C31670 => "C31670",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "C536" => Some(Self::C536),
                "C8387" => Some(Self::C8387),
                "C9673" => Some(Self::C9673),
                "C10285" => Some(Self::C10285),
                "C13318" => Some(Self::C13318),
                "C15963" => Some(Self::C15963),
                "C16439" => Some(Self::C16439),
                "C18197" => Some(Self::C18197),
                "C19430" => Some(Self::C19430),
                "C20361" => Some(Self::C20361),
                "C20706" => Some(Self::C20706),
                "C21050" => Some(Self::C21050),
                "C21906" => Some(Self::C21906),
                "C27265" => Some(Self::C27265),
                "C30109" => Some(Self::C30109),
                "C31670" => Some(Self::C31670),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TestMessageWithManyRepeatedPtrFields<'a> {
    #[femtopb(string, repeated, tag = 1)]
    pub repeated_string_1: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 2)]
    pub repeated_string_2: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 3)]
    pub repeated_string_3: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 4)]
    pub repeated_string_4: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 5)]
    pub repeated_string_5: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 6)]
    pub repeated_string_6: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 7)]
    pub repeated_string_7: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 8)]
    pub repeated_string_8: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 9)]
    pub repeated_string_9: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 10)]
    pub repeated_string_10: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 11)]
    pub repeated_string_11: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 12)]
    pub repeated_string_12: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 13)]
    pub repeated_string_13: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 14)]
    pub repeated_string_14: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 15)]
    pub repeated_string_15: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 16)]
    pub repeated_string_16: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 17)]
    pub repeated_string_17: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 18)]
    pub repeated_string_18: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 19)]
    pub repeated_string_19: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 20)]
    pub repeated_string_20: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 21)]
    pub repeated_string_21: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 22)]
    pub repeated_string_22: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 23)]
    pub repeated_string_23: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 24)]
    pub repeated_string_24: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 25)]
    pub repeated_string_25: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 26)]
    pub repeated_string_26: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 27)]
    pub repeated_string_27: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 28)]
    pub repeated_string_28: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 29)]
    pub repeated_string_29: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 30)]
    pub repeated_string_30: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 31)]
    pub repeated_string_31: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
    #[femtopb(string, repeated, tag = 32)]
    pub repeated_string_32: ::femtopb::repeated::Repeated<
        'a,
        &'a str,
        ::femtopb::item_encoding::String,
    >,
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
pub enum ForeignEnum {
    #[default]
    ForeignFoo = 4,
    ForeignBar = 5,
    ForeignBaz = 6,
    /// (1 << 32) to generate a 64b bitmask would be incorrect.
    ForeignBax = 32,
}
impl ForeignEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ForeignEnum::ForeignFoo => "FOREIGN_FOO",
            ForeignEnum::ForeignBar => "FOREIGN_BAR",
            ForeignEnum::ForeignBaz => "FOREIGN_BAZ",
            ForeignEnum::ForeignBax => "FOREIGN_BAX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FOREIGN_FOO" => Some(Self::ForeignFoo),
            "FOREIGN_BAR" => Some(Self::ForeignBar),
            "FOREIGN_BAZ" => Some(Self::ForeignBaz),
            "FOREIGN_BAX" => Some(Self::ForeignBax),
            _ => None,
        }
    }
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
pub enum TestReservedEnumFields {
    #[default]
    Unknown = 0,
}
impl TestReservedEnumFields {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TestReservedEnumFields::Unknown => "UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            _ => None,
        }
    }
}
/// Test an enum that has multiple values with the same number.
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
pub enum TestEnumWithDupValue {
    #[default]
    Foo1 = 1,
    Bar1 = 2,
    Baz = 3,
}
impl TestEnumWithDupValue {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TestEnumWithDupValue::Foo1 => "FOO1",
            TestEnumWithDupValue::Bar1 => "BAR1",
            TestEnumWithDupValue::Baz => "BAZ",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FOO1" => Some(Self::Foo1),
            "BAR1" => Some(Self::Bar1),
            "BAZ" => Some(Self::Baz),
            _ => None,
        }
    }
}
/// Test an enum with large, unordered values.
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
pub enum TestSparseEnum {
    #[default]
    SparseA = 123,
    SparseB = 62374,
    SparseC = 12589234,
    SparseD = -15,
    SparseE = -53452,
    SparseF = 0,
    SparseG = 2,
}
impl TestSparseEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TestSparseEnum::SparseA => "SPARSE_A",
            TestSparseEnum::SparseB => "SPARSE_B",
            TestSparseEnum::SparseC => "SPARSE_C",
            TestSparseEnum::SparseD => "SPARSE_D",
            TestSparseEnum::SparseE => "SPARSE_E",
            TestSparseEnum::SparseF => "SPARSE_F",
            TestSparseEnum::SparseG => "SPARSE_G",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SPARSE_A" => Some(Self::SparseA),
            "SPARSE_B" => Some(Self::SparseB),
            "SPARSE_C" => Some(Self::SparseC),
            "SPARSE_D" => Some(Self::SparseD),
            "SPARSE_E" => Some(Self::SparseE),
            "SPARSE_F" => Some(Self::SparseF),
            "SPARSE_G" => Some(Self::SparseG),
            _ => None,
        }
    }
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
pub enum VeryLargeEnum {
    #[default]
    EnumLabelDefault = 0,
    EnumLabel1 = 1,
    EnumLabel2 = 2,
    EnumLabel3 = 3,
    EnumLabel4 = 4,
    EnumLabel5 = 5,
    EnumLabel6 = 6,
    EnumLabel7 = 7,
    EnumLabel8 = 8,
    EnumLabel9 = 9,
    EnumLabel10 = 10,
    EnumLabel11 = 11,
    EnumLabel12 = 12,
    EnumLabel13 = 13,
    EnumLabel14 = 14,
    EnumLabel15 = 15,
    EnumLabel16 = 16,
    EnumLabel17 = 17,
    EnumLabel18 = 18,
    EnumLabel19 = 19,
    EnumLabel20 = 20,
    EnumLabel21 = 21,
    EnumLabel22 = 22,
    EnumLabel23 = 23,
    EnumLabel24 = 24,
    EnumLabel25 = 25,
    EnumLabel26 = 26,
    EnumLabel27 = 27,
    EnumLabel28 = 28,
    EnumLabel29 = 29,
    EnumLabel30 = 30,
    EnumLabel31 = 31,
    EnumLabel32 = 32,
    EnumLabel33 = 33,
    EnumLabel34 = 34,
    EnumLabel35 = 35,
    EnumLabel36 = 36,
    EnumLabel37 = 37,
    EnumLabel38 = 38,
    EnumLabel39 = 39,
    EnumLabel40 = 40,
    EnumLabel41 = 41,
    EnumLabel42 = 42,
    EnumLabel43 = 43,
    EnumLabel44 = 44,
    EnumLabel45 = 45,
    EnumLabel46 = 46,
    EnumLabel47 = 47,
    EnumLabel48 = 48,
    EnumLabel49 = 49,
    EnumLabel50 = 50,
    EnumLabel51 = 51,
    EnumLabel52 = 52,
    EnumLabel53 = 53,
    EnumLabel54 = 54,
    EnumLabel55 = 55,
    EnumLabel56 = 56,
    EnumLabel57 = 57,
    EnumLabel58 = 58,
    EnumLabel59 = 59,
    EnumLabel60 = 60,
    EnumLabel61 = 61,
    EnumLabel62 = 62,
    EnumLabel63 = 63,
    EnumLabel64 = 64,
    EnumLabel65 = 65,
    EnumLabel66 = 66,
    EnumLabel67 = 67,
    EnumLabel68 = 68,
    EnumLabel69 = 69,
    EnumLabel70 = 70,
    EnumLabel71 = 71,
    EnumLabel72 = 72,
    EnumLabel73 = 73,
    EnumLabel74 = 74,
    EnumLabel75 = 75,
    EnumLabel76 = 76,
    EnumLabel77 = 77,
    EnumLabel78 = 78,
    EnumLabel79 = 79,
    EnumLabel80 = 80,
    EnumLabel81 = 81,
    EnumLabel82 = 82,
    EnumLabel83 = 83,
    EnumLabel84 = 84,
    EnumLabel85 = 85,
    EnumLabel86 = 86,
    EnumLabel87 = 87,
    EnumLabel88 = 88,
    EnumLabel89 = 89,
    EnumLabel90 = 90,
    EnumLabel91 = 91,
    EnumLabel92 = 92,
    EnumLabel93 = 93,
    EnumLabel94 = 94,
    EnumLabel95 = 95,
    EnumLabel96 = 96,
    EnumLabel97 = 97,
    EnumLabel98 = 98,
    EnumLabel99 = 99,
    EnumLabel100 = 100,
}
impl VeryLargeEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VeryLargeEnum::EnumLabelDefault => "ENUM_LABEL_DEFAULT",
            VeryLargeEnum::EnumLabel1 => "ENUM_LABEL_1",
            VeryLargeEnum::EnumLabel2 => "ENUM_LABEL_2",
            VeryLargeEnum::EnumLabel3 => "ENUM_LABEL_3",
            VeryLargeEnum::EnumLabel4 => "ENUM_LABEL_4",
            VeryLargeEnum::EnumLabel5 => "ENUM_LABEL_5",
            VeryLargeEnum::EnumLabel6 => "ENUM_LABEL_6",
            VeryLargeEnum::EnumLabel7 => "ENUM_LABEL_7",
            VeryLargeEnum::EnumLabel8 => "ENUM_LABEL_8",
            VeryLargeEnum::EnumLabel9 => "ENUM_LABEL_9",
            VeryLargeEnum::EnumLabel10 => "ENUM_LABEL_10",
            VeryLargeEnum::EnumLabel11 => "ENUM_LABEL_11",
            VeryLargeEnum::EnumLabel12 => "ENUM_LABEL_12",
            VeryLargeEnum::EnumLabel13 => "ENUM_LABEL_13",
            VeryLargeEnum::EnumLabel14 => "ENUM_LABEL_14",
            VeryLargeEnum::EnumLabel15 => "ENUM_LABEL_15",
            VeryLargeEnum::EnumLabel16 => "ENUM_LABEL_16",
            VeryLargeEnum::EnumLabel17 => "ENUM_LABEL_17",
            VeryLargeEnum::EnumLabel18 => "ENUM_LABEL_18",
            VeryLargeEnum::EnumLabel19 => "ENUM_LABEL_19",
            VeryLargeEnum::EnumLabel20 => "ENUM_LABEL_20",
            VeryLargeEnum::EnumLabel21 => "ENUM_LABEL_21",
            VeryLargeEnum::EnumLabel22 => "ENUM_LABEL_22",
            VeryLargeEnum::EnumLabel23 => "ENUM_LABEL_23",
            VeryLargeEnum::EnumLabel24 => "ENUM_LABEL_24",
            VeryLargeEnum::EnumLabel25 => "ENUM_LABEL_25",
            VeryLargeEnum::EnumLabel26 => "ENUM_LABEL_26",
            VeryLargeEnum::EnumLabel27 => "ENUM_LABEL_27",
            VeryLargeEnum::EnumLabel28 => "ENUM_LABEL_28",
            VeryLargeEnum::EnumLabel29 => "ENUM_LABEL_29",
            VeryLargeEnum::EnumLabel30 => "ENUM_LABEL_30",
            VeryLargeEnum::EnumLabel31 => "ENUM_LABEL_31",
            VeryLargeEnum::EnumLabel32 => "ENUM_LABEL_32",
            VeryLargeEnum::EnumLabel33 => "ENUM_LABEL_33",
            VeryLargeEnum::EnumLabel34 => "ENUM_LABEL_34",
            VeryLargeEnum::EnumLabel35 => "ENUM_LABEL_35",
            VeryLargeEnum::EnumLabel36 => "ENUM_LABEL_36",
            VeryLargeEnum::EnumLabel37 => "ENUM_LABEL_37",
            VeryLargeEnum::EnumLabel38 => "ENUM_LABEL_38",
            VeryLargeEnum::EnumLabel39 => "ENUM_LABEL_39",
            VeryLargeEnum::EnumLabel40 => "ENUM_LABEL_40",
            VeryLargeEnum::EnumLabel41 => "ENUM_LABEL_41",
            VeryLargeEnum::EnumLabel42 => "ENUM_LABEL_42",
            VeryLargeEnum::EnumLabel43 => "ENUM_LABEL_43",
            VeryLargeEnum::EnumLabel44 => "ENUM_LABEL_44",
            VeryLargeEnum::EnumLabel45 => "ENUM_LABEL_45",
            VeryLargeEnum::EnumLabel46 => "ENUM_LABEL_46",
            VeryLargeEnum::EnumLabel47 => "ENUM_LABEL_47",
            VeryLargeEnum::EnumLabel48 => "ENUM_LABEL_48",
            VeryLargeEnum::EnumLabel49 => "ENUM_LABEL_49",
            VeryLargeEnum::EnumLabel50 => "ENUM_LABEL_50",
            VeryLargeEnum::EnumLabel51 => "ENUM_LABEL_51",
            VeryLargeEnum::EnumLabel52 => "ENUM_LABEL_52",
            VeryLargeEnum::EnumLabel53 => "ENUM_LABEL_53",
            VeryLargeEnum::EnumLabel54 => "ENUM_LABEL_54",
            VeryLargeEnum::EnumLabel55 => "ENUM_LABEL_55",
            VeryLargeEnum::EnumLabel56 => "ENUM_LABEL_56",
            VeryLargeEnum::EnumLabel57 => "ENUM_LABEL_57",
            VeryLargeEnum::EnumLabel58 => "ENUM_LABEL_58",
            VeryLargeEnum::EnumLabel59 => "ENUM_LABEL_59",
            VeryLargeEnum::EnumLabel60 => "ENUM_LABEL_60",
            VeryLargeEnum::EnumLabel61 => "ENUM_LABEL_61",
            VeryLargeEnum::EnumLabel62 => "ENUM_LABEL_62",
            VeryLargeEnum::EnumLabel63 => "ENUM_LABEL_63",
            VeryLargeEnum::EnumLabel64 => "ENUM_LABEL_64",
            VeryLargeEnum::EnumLabel65 => "ENUM_LABEL_65",
            VeryLargeEnum::EnumLabel66 => "ENUM_LABEL_66",
            VeryLargeEnum::EnumLabel67 => "ENUM_LABEL_67",
            VeryLargeEnum::EnumLabel68 => "ENUM_LABEL_68",
            VeryLargeEnum::EnumLabel69 => "ENUM_LABEL_69",
            VeryLargeEnum::EnumLabel70 => "ENUM_LABEL_70",
            VeryLargeEnum::EnumLabel71 => "ENUM_LABEL_71",
            VeryLargeEnum::EnumLabel72 => "ENUM_LABEL_72",
            VeryLargeEnum::EnumLabel73 => "ENUM_LABEL_73",
            VeryLargeEnum::EnumLabel74 => "ENUM_LABEL_74",
            VeryLargeEnum::EnumLabel75 => "ENUM_LABEL_75",
            VeryLargeEnum::EnumLabel76 => "ENUM_LABEL_76",
            VeryLargeEnum::EnumLabel77 => "ENUM_LABEL_77",
            VeryLargeEnum::EnumLabel78 => "ENUM_LABEL_78",
            VeryLargeEnum::EnumLabel79 => "ENUM_LABEL_79",
            VeryLargeEnum::EnumLabel80 => "ENUM_LABEL_80",
            VeryLargeEnum::EnumLabel81 => "ENUM_LABEL_81",
            VeryLargeEnum::EnumLabel82 => "ENUM_LABEL_82",
            VeryLargeEnum::EnumLabel83 => "ENUM_LABEL_83",
            VeryLargeEnum::EnumLabel84 => "ENUM_LABEL_84",
            VeryLargeEnum::EnumLabel85 => "ENUM_LABEL_85",
            VeryLargeEnum::EnumLabel86 => "ENUM_LABEL_86",
            VeryLargeEnum::EnumLabel87 => "ENUM_LABEL_87",
            VeryLargeEnum::EnumLabel88 => "ENUM_LABEL_88",
            VeryLargeEnum::EnumLabel89 => "ENUM_LABEL_89",
            VeryLargeEnum::EnumLabel90 => "ENUM_LABEL_90",
            VeryLargeEnum::EnumLabel91 => "ENUM_LABEL_91",
            VeryLargeEnum::EnumLabel92 => "ENUM_LABEL_92",
            VeryLargeEnum::EnumLabel93 => "ENUM_LABEL_93",
            VeryLargeEnum::EnumLabel94 => "ENUM_LABEL_94",
            VeryLargeEnum::EnumLabel95 => "ENUM_LABEL_95",
            VeryLargeEnum::EnumLabel96 => "ENUM_LABEL_96",
            VeryLargeEnum::EnumLabel97 => "ENUM_LABEL_97",
            VeryLargeEnum::EnumLabel98 => "ENUM_LABEL_98",
            VeryLargeEnum::EnumLabel99 => "ENUM_LABEL_99",
            VeryLargeEnum::EnumLabel100 => "ENUM_LABEL_100",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENUM_LABEL_DEFAULT" => Some(Self::EnumLabelDefault),
            "ENUM_LABEL_1" => Some(Self::EnumLabel1),
            "ENUM_LABEL_2" => Some(Self::EnumLabel2),
            "ENUM_LABEL_3" => Some(Self::EnumLabel3),
            "ENUM_LABEL_4" => Some(Self::EnumLabel4),
            "ENUM_LABEL_5" => Some(Self::EnumLabel5),
            "ENUM_LABEL_6" => Some(Self::EnumLabel6),
            "ENUM_LABEL_7" => Some(Self::EnumLabel7),
            "ENUM_LABEL_8" => Some(Self::EnumLabel8),
            "ENUM_LABEL_9" => Some(Self::EnumLabel9),
            "ENUM_LABEL_10" => Some(Self::EnumLabel10),
            "ENUM_LABEL_11" => Some(Self::EnumLabel11),
            "ENUM_LABEL_12" => Some(Self::EnumLabel12),
            "ENUM_LABEL_13" => Some(Self::EnumLabel13),
            "ENUM_LABEL_14" => Some(Self::EnumLabel14),
            "ENUM_LABEL_15" => Some(Self::EnumLabel15),
            "ENUM_LABEL_16" => Some(Self::EnumLabel16),
            "ENUM_LABEL_17" => Some(Self::EnumLabel17),
            "ENUM_LABEL_18" => Some(Self::EnumLabel18),
            "ENUM_LABEL_19" => Some(Self::EnumLabel19),
            "ENUM_LABEL_20" => Some(Self::EnumLabel20),
            "ENUM_LABEL_21" => Some(Self::EnumLabel21),
            "ENUM_LABEL_22" => Some(Self::EnumLabel22),
            "ENUM_LABEL_23" => Some(Self::EnumLabel23),
            "ENUM_LABEL_24" => Some(Self::EnumLabel24),
            "ENUM_LABEL_25" => Some(Self::EnumLabel25),
            "ENUM_LABEL_26" => Some(Self::EnumLabel26),
            "ENUM_LABEL_27" => Some(Self::EnumLabel27),
            "ENUM_LABEL_28" => Some(Self::EnumLabel28),
            "ENUM_LABEL_29" => Some(Self::EnumLabel29),
            "ENUM_LABEL_30" => Some(Self::EnumLabel30),
            "ENUM_LABEL_31" => Some(Self::EnumLabel31),
            "ENUM_LABEL_32" => Some(Self::EnumLabel32),
            "ENUM_LABEL_33" => Some(Self::EnumLabel33),
            "ENUM_LABEL_34" => Some(Self::EnumLabel34),
            "ENUM_LABEL_35" => Some(Self::EnumLabel35),
            "ENUM_LABEL_36" => Some(Self::EnumLabel36),
            "ENUM_LABEL_37" => Some(Self::EnumLabel37),
            "ENUM_LABEL_38" => Some(Self::EnumLabel38),
            "ENUM_LABEL_39" => Some(Self::EnumLabel39),
            "ENUM_LABEL_40" => Some(Self::EnumLabel40),
            "ENUM_LABEL_41" => Some(Self::EnumLabel41),
            "ENUM_LABEL_42" => Some(Self::EnumLabel42),
            "ENUM_LABEL_43" => Some(Self::EnumLabel43),
            "ENUM_LABEL_44" => Some(Self::EnumLabel44),
            "ENUM_LABEL_45" => Some(Self::EnumLabel45),
            "ENUM_LABEL_46" => Some(Self::EnumLabel46),
            "ENUM_LABEL_47" => Some(Self::EnumLabel47),
            "ENUM_LABEL_48" => Some(Self::EnumLabel48),
            "ENUM_LABEL_49" => Some(Self::EnumLabel49),
            "ENUM_LABEL_50" => Some(Self::EnumLabel50),
            "ENUM_LABEL_51" => Some(Self::EnumLabel51),
            "ENUM_LABEL_52" => Some(Self::EnumLabel52),
            "ENUM_LABEL_53" => Some(Self::EnumLabel53),
            "ENUM_LABEL_54" => Some(Self::EnumLabel54),
            "ENUM_LABEL_55" => Some(Self::EnumLabel55),
            "ENUM_LABEL_56" => Some(Self::EnumLabel56),
            "ENUM_LABEL_57" => Some(Self::EnumLabel57),
            "ENUM_LABEL_58" => Some(Self::EnumLabel58),
            "ENUM_LABEL_59" => Some(Self::EnumLabel59),
            "ENUM_LABEL_60" => Some(Self::EnumLabel60),
            "ENUM_LABEL_61" => Some(Self::EnumLabel61),
            "ENUM_LABEL_62" => Some(Self::EnumLabel62),
            "ENUM_LABEL_63" => Some(Self::EnumLabel63),
            "ENUM_LABEL_64" => Some(Self::EnumLabel64),
            "ENUM_LABEL_65" => Some(Self::EnumLabel65),
            "ENUM_LABEL_66" => Some(Self::EnumLabel66),
            "ENUM_LABEL_67" => Some(Self::EnumLabel67),
            "ENUM_LABEL_68" => Some(Self::EnumLabel68),
            "ENUM_LABEL_69" => Some(Self::EnumLabel69),
            "ENUM_LABEL_70" => Some(Self::EnumLabel70),
            "ENUM_LABEL_71" => Some(Self::EnumLabel71),
            "ENUM_LABEL_72" => Some(Self::EnumLabel72),
            "ENUM_LABEL_73" => Some(Self::EnumLabel73),
            "ENUM_LABEL_74" => Some(Self::EnumLabel74),
            "ENUM_LABEL_75" => Some(Self::EnumLabel75),
            "ENUM_LABEL_76" => Some(Self::EnumLabel76),
            "ENUM_LABEL_77" => Some(Self::EnumLabel77),
            "ENUM_LABEL_78" => Some(Self::EnumLabel78),
            "ENUM_LABEL_79" => Some(Self::EnumLabel79),
            "ENUM_LABEL_80" => Some(Self::EnumLabel80),
            "ENUM_LABEL_81" => Some(Self::EnumLabel81),
            "ENUM_LABEL_82" => Some(Self::EnumLabel82),
            "ENUM_LABEL_83" => Some(Self::EnumLabel83),
            "ENUM_LABEL_84" => Some(Self::EnumLabel84),
            "ENUM_LABEL_85" => Some(Self::EnumLabel85),
            "ENUM_LABEL_86" => Some(Self::EnumLabel86),
            "ENUM_LABEL_87" => Some(Self::EnumLabel87),
            "ENUM_LABEL_88" => Some(Self::EnumLabel88),
            "ENUM_LABEL_89" => Some(Self::EnumLabel89),
            "ENUM_LABEL_90" => Some(Self::EnumLabel90),
            "ENUM_LABEL_91" => Some(Self::EnumLabel91),
            "ENUM_LABEL_92" => Some(Self::EnumLabel92),
            "ENUM_LABEL_93" => Some(Self::EnumLabel93),
            "ENUM_LABEL_94" => Some(Self::EnumLabel94),
            "ENUM_LABEL_95" => Some(Self::EnumLabel95),
            "ENUM_LABEL_96" => Some(Self::EnumLabel96),
            "ENUM_LABEL_97" => Some(Self::EnumLabel97),
            "ENUM_LABEL_98" => Some(Self::EnumLabel98),
            "ENUM_LABEL_99" => Some(Self::EnumLabel99),
            "ENUM_LABEL_100" => Some(Self::EnumLabel100),
            _ => None,
        }
    }
}
