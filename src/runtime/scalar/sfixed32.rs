crate::runtime::macros::fixed_width!(
    'a,
    i32,
    crate::item_encoding::SFixed32,
    4,
    encoding::WireType::ThirtyTwoBit,
    put_i32_le,
    get_i32_le
);