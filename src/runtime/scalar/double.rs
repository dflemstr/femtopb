crate::runtime::macros::fixed_width!(
    'a,
    f64,
    crate::item_encoding::Double,
    8,
    encoding::WireType::SixtyFourBit,
    put_f64_le,
    get_f64_le
);
