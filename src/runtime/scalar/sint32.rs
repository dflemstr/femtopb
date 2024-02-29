crate::runtime::macros::varint!('a, i32, crate::item_encoding::SInt32,
    to_uint64: |value| {
        ((value << 1) ^ (value >> 31)) as u32 as u64
    },
    from_uint64: |value| {
        let value = value as u32;
        ((value >> 1) as i32) ^ (-((value & 1) as i32))
    }
);
