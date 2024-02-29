crate::runtime::macros::varint!('a, bool, crate::item_encoding::Bool,
    to_uint64: |value| u64::from(value),
    from_uint64: |value| value != 0);
