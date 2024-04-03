/// Helper macro which emits an `encode_repeated` function for the type.
macro_rules! encode_repeated {
    ($lt:lifetime, $ty:ty, $item_encoding:ty) => {
        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encode_repeated<$lt>(
            tag: u32,
            values: $crate::repeated::Repeated<$lt, $ty, $item_encoding>,
            cursor: &mut &mut [u8],
        ) {
            for result in values {
                if let Ok(value) = result {
                    encode_key_value(tag, value, cursor);
                }
            }
        }
    };
}
pub(crate) use encode_repeated;

/// Helper macro which emits the `clear` functions using trivial logic
/// (i.e. set to default values/empty values)
macro_rules! trivial_clear {
    ($lt:lifetime, $ty:ty, $default_ty:ty, $item_encoding:ty) => {
        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn clear<$lt>(value: &mut $ty, default: $default_ty) {
            *value = default;
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn clear_optional<$lt>(value: &mut Option<$ty>, default: Option<$default_ty>) {
            *value = default;
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn clear_repeated<$lt>(
            value: &mut $crate::repeated::Repeated<$lt, $ty, $item_encoding>,
        ) {
            *value = $crate::repeated::Repeated::empty();
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn clear_packed<$lt>(value: &mut $crate::packed::Packed<$lt, $ty, $item_encoding>) {
            *value = $crate::packed::Packed::empty();
        }
    };
}
pub(crate) use trivial_clear;

/// Helper macro which emits the `decode_packed` and `decode_repeated` functions.
macro_rules! decode_packed_repeated {
    ($lt:lifetime, $ty:ty, $item_encoding:ty) => {
        use $crate::packed;
        use $crate::repeated;

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn decode_repeated<$lt>(
            tag: u32,
            wire_type: encoding::WireType,
            msg_buf: &$lt [u8],
            cursor: &mut &$lt [u8],
            field: &mut repeated::Repeated<$lt, $ty, $item_encoding>,
        ) -> Result<(), error::DecodeError> {
            if field.is_unpopulated() {
                *field = repeated::Repeated::from_msg_buf(tag, msg_buf);
            }
            encoding::skip_field(wire_type, tag, cursor)?;
            Ok(())
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn decode_packed<$lt>(
            tag: u32,
            wire_type: encoding::WireType,
            msg_buf: &$lt [u8],
            cursor: &mut &$lt [u8],
            field: &mut packed::Packed<$lt, $ty, $item_encoding>,
        ) -> Result<(), error::DecodeError> {
            if field.is_unpopulated() {
                *field = packed::Packed::from_msg_buf(tag, msg_buf);
            }
            encoding::skip_field(wire_type, tag, cursor)?;
            Ok(())
        }
    };
}
pub(crate) use decode_packed_repeated;

/// Macro which emits a set of encoding functions for a variable width numeric type.
macro_rules! varint {
    ($lt:lifetime, $ty:ty, $item_encoding:ty) => (
        $crate::runtime::macros::varint!($lt, $ty, $item_encoding,
                to_uint64: |value| { value as u64 },
                from_uint64: |value| { value as $ty });
    );

    ($lt:lifetime, $ty:ty, $item_encoding:ty,
     to_uint64: |$to_uint64_value:ident| $to_uint64:expr,
     from_uint64: |$from_uint64_value:ident| $from_uint64:expr) => (
        use $crate::encoding;
        use $crate::error;

        // Also used by `crate::item_encoding`
        pub(crate) const WIRE_TYPE: encoding::WireType = encoding::WireType::Varint;

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encode(tag: u32, value: $ty, default: $ty, cursor: &mut &mut [u8]) {
            if value != default {
                encode_key_value(tag, value, cursor);
            }
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encode_optional(tag: u32, value: Option<$ty>, default: $ty, cursor: &mut &mut [u8]) {
            if let Some(value) = value {
                if value != default {
                    encode_key_value(tag, value, cursor);
                }
            }
        }

        $crate::runtime::macros::encode_repeated!($lt, $ty, $item_encoding);

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encode_packed<$lt>(tag: u32, values: $crate::packed::Packed<$lt, $ty, $item_encoding>, cursor: &mut &mut [u8]) {
            if !values.is_empty() {
                encoding::encode_key(tag, encoding::WireType::LengthDelimited, cursor);
                let len: usize = values.iter().map(|r| {
                    r.map(|$to_uint64_value| {
                        encoding::encoded_len_varint($to_uint64)
                    }).unwrap_or(0)
                }).sum();
                encoding::encode_varint(len as u64, cursor);

                for r in values {
                    if let Ok(value) = r {
                        encode_single_value(value, cursor);
                    }
                }
            }
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        fn encode_key_value(tag: u32, value: $ty, cursor: &mut &mut [u8]) {
            encoding::encode_key(tag, WIRE_TYPE, cursor);
            encode_single_value(value, cursor);
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        fn encode_single_value($to_uint64_value: $ty, cursor: &mut &mut [u8]) {
            encoding::encode_varint($to_uint64, cursor);
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encoded_len(tag: u32, $to_uint64_value: $ty, default: $ty) -> usize {
            if $to_uint64_value == default {
                0
            } else {
                encoding::key_len(tag) + encoding::encoded_len_varint($to_uint64)
            }
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encoded_len_optional(tag: u32, v: Option<$ty>, default: $ty) -> usize {
            if let Some(v) = v {
                encoded_len(tag, v, default)
            } else {
                0
            }
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encoded_len_repeated<$lt>(tag: u32, values: $crate::repeated::Repeated<$lt, $ty, $item_encoding>) -> usize {
            encoding::key_len(tag) * values.len() + values.iter().map(|r| {
                r.map(|$to_uint64_value| encoding::encoded_len_varint($to_uint64)).unwrap_or(0)
            }).sum::<usize>()
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encoded_len_packed<$lt>(tag: u32, values: $crate::packed::Packed<$lt, $ty, $item_encoding>) -> usize {
            if values.is_empty() {
                0
            } else {
                let len = values.iter()
                                .map(|r| { r.map(|$to_uint64_value| encoding::encoded_len_varint($to_uint64)).unwrap_or(0) })
                                .sum::<usize>();
                encoding::key_len(tag) + encoding::encoded_len_varint(len as u64) + len
            }
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn decode(_tag: u32, wire_type: encoding::WireType, _msg_buf: &[u8], cursor: &mut &[u8], field: &mut $ty) -> Result<(), error::DecodeError> {
            encoding::check_wire_type(WIRE_TYPE, wire_type)?;
            *field = decode_single_value(cursor)?;
            Ok(())
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn decode_optional(_tag: u32, wire_type: encoding::WireType, _msg_buf: &[u8], cursor: &mut &[u8], field: &mut Option<$ty>) -> Result<(), error::DecodeError> {
            encoding::check_wire_type(WIRE_TYPE, wire_type)?;
            *field = Some(decode_single_value(cursor)?);
            Ok(())
        }

        // Also used by `crate::item_encoding`
        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub(crate) fn decode_single_value(cursor: &mut &[u8]) -> Result<$ty, error::DecodeError> {
            let $from_uint64_value = encoding::decode_varint(cursor)?;
            Ok($from_uint64)
        }

        $crate::runtime::macros::decode_packed_repeated!($lt, $ty, $item_encoding);
        $crate::runtime::macros::trivial_clear!($lt, $ty, $ty, $item_encoding);
    );
}
pub(crate) use varint;

/// Macro which emits a set of encoding functions for a fixed width numeric type.
macro_rules! fixed_width {
    ($lt:lifetime,
     $ty:ty,
     $item_encoding:ty,
     $width:expr,
     $wire_type:expr,
     $put:ident,
     $get:ident) => {
        use $crate::encoding;
        use $crate::error;
        use $crate::bits;

        // Also used by `crate::item_encoding`
        pub(crate) const WIRE_TYPE: encoding::WireType = $wire_type;

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encode(tag: u32, value: $ty, default: $ty, cursor: &mut &mut [u8]) {
            if value != default {
                encode_key_value(tag, value, cursor);
            }
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encode_optional(tag: u32, value: Option<$ty>, default: $ty, cursor: &mut &mut [u8]) {
            if let Some(value) = value {
                if value != default {
                    encode_key_value(tag, value, cursor);
                }
            }
        }

        $crate::runtime::macros::encode_repeated!($lt, $ty, $item_encoding);

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encode_packed(
            tag: u32,
            values: $crate::packed::Packed<$ty, $item_encoding>,
            cursor: &mut &mut [u8],
        ) {
            if !values.is_empty() {
                encoding::encode_key(tag, encoding::WireType::LengthDelimited, cursor);
                let len = values.len() as u64 * $width;
                encoding::encode_varint(len as u64, cursor);

                for result in values {
                    if let Ok(value) = result {
                        encode_single_value(value, cursor);
                    }
                }
            }
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        fn encode_key_value(tag: u32, value: $ty, cursor: &mut &mut [u8]) {
            encoding::encode_key(tag, WIRE_TYPE, cursor);
            encode_single_value(value, cursor);
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        fn encode_single_value(value: $ty, cursor: &mut &mut [u8]) {
            bits::$put(cursor, value);
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encoded_len(tag: u32, value: $ty, default: $ty) -> usize {
            if value == default {
                0
            } else {
                encoding::key_len(tag) + $width
            }
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encoded_len_optional(tag: u32, v: Option<$ty>, default: $ty) -> usize {
            if let Some(v) = v {
                encoded_len(tag, v, default)
            } else {
                0
            }
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encoded_len_repeated(
            tag: u32,
            values: $crate::repeated::Repeated<$ty, $item_encoding>,
        ) -> usize {
            (encoding::key_len(tag) + $width) * values.len()
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encoded_len_packed(
            tag: u32,
            values: $crate::packed::Packed<$ty, $item_encoding>,
        ) -> usize {
            if values.is_empty() {
                0
            } else {
                let len = $width * values.len();
                encoding::key_len(tag) + encoding::encoded_len_varint(len as u64) + len
            }
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn decode<$lt>(
            _tag: u32,
            wire_type: encoding::WireType,
            _msg_buf: &$lt [u8],
            cursor: &mut &$lt [u8],
            field: &mut $ty,
        ) -> Result<(), error::DecodeError> {
            encoding::check_wire_type(WIRE_TYPE, wire_type)?;
            *field = decode_single_value(cursor)?;
            Ok(())
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn decode_optional<$lt>(
            _tag: u32,
            wire_type: encoding::WireType,
            _msg_buf: &$lt[u8],
            cursor: &mut &$lt[u8],
            field: &mut Option<$ty>,
        ) -> Result<(), error::DecodeError> {
            encoding::check_wire_type(WIRE_TYPE, wire_type)?;
            *field = Some(decode_single_value(cursor)?);
            Ok(())
        }

        // Also used by `crate::item_encoding`
        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub(crate) fn decode_single_value(cursor: &mut &[u8]) -> Result<$ty, error::DecodeError> {
            if cursor.len() >= $width {
                Ok(bits::$get(cursor))
            } else {
                Err(error::DecodeError::BufferUnderflow)
            }
        }

        $crate::runtime::macros::decode_packed_repeated!($lt, $ty, $item_encoding);
        $crate::runtime::macros::trivial_clear!($lt, $ty, $ty, $item_encoding);
    };
}
pub(crate) use fixed_width;

/// Macro which emits encoding functions for a length-delimited type.
macro_rules! length_delimited {
    ($lt:lifetime, $ty:ty, $item_encoding:ty) => {
        $crate::runtime::macros::encode_repeated!($lt, $ty, $item_encoding);

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encoded_len<$lt>(tag: u32, value: $ty, default: $ty) -> usize {
            use $crate::encoding;
            if value == default {
                0
            } else {
                encoding::key_len(tag)
                    + encoding::encoded_len_varint(value.len() as u64)
                    + value.len()
            }
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encoded_len_optional<$lt>(tag: u32, value: Option<$ty>, default: $ty) -> usize {
            if let Some(v) = value {
                encoded_len(tag, v, default)
            } else {
                0
            }
        }

        #[inline]
        #[cfg_attr(feature = "assert-no-panic", no_panic::no_panic)]
        pub fn encoded_len_repeated<$lt>(
            tag: u32,
            values: $crate::repeated::Repeated<$lt, $ty, $item_encoding>,
        ) -> usize {
            use $crate::encoding;
            encoding::key_len(tag) * values.len()
                + values
                    .iter()
                    .map(|r| {
                        r.map(|v| encoding::encoded_len_varint(v.len() as u64) + v.len())
                            .unwrap_or(0)
                    })
                    .sum::<usize>()
        }
    };
}
pub(crate) use length_delimited;
