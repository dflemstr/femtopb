crate::runtime::macros::varint!('a, i64, crate::item_encoding::SInt64,
    to_uint64: |value| {
        ((value << 1) ^ (value >> 63)) as u64
    },
    from_uint64: |value| {
        ((value >> 1) as i64) ^ (-((value & 1) as i64))
    }
);
