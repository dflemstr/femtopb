use core::mem;

fn put_chunk<const N: usize>(cursor: &mut &mut [u8], data: [u8; N]) {
    let buf = mem::take(cursor);
    let (chunk, rest) = buf.split_first_chunk_mut::<N>().unwrap();
    *chunk = data;
    *cursor = rest;
}

fn take_chunk<const N: usize>(cursor: &mut &[u8]) -> [u8; N] {
    let (&chunk, rest) = cursor.split_first_chunk().unwrap();
    *cursor = rest;
    chunk
}

pub fn put_f32_le(cursor: &mut &mut [u8], value: f32) {
    put_chunk(cursor, value.to_le_bytes());
}

pub fn get_f32_le(cursor: &mut &[u8]) -> f32 {
    f32::from_le_bytes(take_chunk(cursor))
}

pub fn put_f64_le(cursor: &mut &mut [u8], value: f64) {
    put_chunk(cursor, value.to_le_bytes());
}

pub fn get_f64_le(cursor: &mut &[u8]) -> f64 {
    f64::from_le_bytes(take_chunk(cursor))
}

pub fn put_u32_le(cursor: &mut &mut [u8], value: u32) {
    put_chunk(cursor, value.to_le_bytes());
}

pub fn get_u32_le(cursor: &mut &[u8]) -> u32 {
    u32::from_le_bytes(take_chunk(cursor))
}

pub fn put_u64_le(cursor: &mut &mut [u8], value: u64) {
    put_chunk(cursor, value.to_le_bytes());
}

pub fn get_u64_le(cursor: &mut &[u8]) -> u64 {
    u64::from_le_bytes(take_chunk(cursor))
}

pub fn put_i32_le(cursor: &mut &mut [u8], value: i32) {
    put_chunk(cursor, value.to_le_bytes());
}

pub fn get_i32_le(cursor: &mut &[u8]) -> i32 {
    i32::from_le_bytes(take_chunk(cursor))
}

pub fn put_i64_le(cursor: &mut &mut [u8], value: i64) {
    put_chunk(cursor, value.to_le_bytes());
}

pub fn get_i64_le(cursor: &mut &[u8]) -> i64 {
    i64::from_le_bytes(take_chunk(cursor))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes `value` into a fresh `N`-byte buffer via `put`, returns the bytes, and asserts the
    /// write cursor was advanced to the end.
    fn write<const N: usize>(put: impl FnOnce(&mut &mut [u8]), _n: usize) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut cursor: &mut [u8] = &mut buf;
        put(&mut cursor);
        assert!(cursor.is_empty(), "put must advance the cursor to the end");
        buf
    }

    #[test]
    fn little_endian_byte_layout() {
        assert_eq!(write::<4>(|c| put_u32_le(c, 0x0403_0201), 4), [1, 2, 3, 4]);
        assert_eq!(
            write::<8>(|c| put_u64_le(c, 0x0807_0605_0403_0201), 8),
            [1, 2, 3, 4, 5, 6, 7, 8]
        );
        assert_eq!(
            write::<4>(|c| put_i32_le(c, -1), 4),
            [0xFF, 0xFF, 0xFF, 0xFF]
        );
        // f32 1.0 == 0x3F800000 little-endian.
        assert_eq!(
            write::<4>(|c| put_f32_le(c, 1.0), 4),
            [0x00, 0x00, 0x80, 0x3F]
        );
    }

    #[test]
    fn round_trips_each_type() {
        macro_rules! check {
            ($put:ident, $get:ident, $n:literal, $val:expr) => {{
                let bytes = write::<$n>(|c| $put(c, $val), $n);
                let mut cursor: &[u8] = &bytes;
                assert_eq!($get(&mut cursor), $val);
                assert!(cursor.is_empty(), "get must advance the cursor to the end");
            }};
        }
        check!(put_u32_le, get_u32_le, 4, 0xDEAD_BEEFu32);
        check!(put_u64_le, get_u64_le, 8, 0x0123_4567_89AB_CDEFu64);
        check!(put_i32_le, get_i32_le, 4, i32::MIN);
        check!(put_i32_le, get_i32_le, 4, i32::MAX);
        check!(put_i64_le, get_i64_le, 8, i64::MIN);
        check!(put_i64_le, get_i64_le, 8, i64::MAX);
        check!(put_f32_le, get_f32_le, 4, core::f32::consts::PI);
        check!(put_f64_le, get_f64_le, 8, core::f64::consts::E);
    }

    #[test]
    fn get_advances_cursor_and_leaves_the_rest() {
        let buf = [1u8, 0, 0, 0, 0xAA, 0xBB];
        let mut cursor: &[u8] = &buf;
        assert_eq!(get_u32_le(&mut cursor), 1);
        assert_eq!(cursor, &[0xAA, 0xBB]); // trailing bytes untouched, cursor advanced by 4
    }

    #[test]
    fn float_special_values_are_bit_preserved() {
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -0.0f32] {
            let bytes = write::<4>(|c| put_f32_le(c, value), 4);
            let mut cursor: &[u8] = &bytes;
            // Compare bit patterns: NaN != NaN and -0.0 == 0.0 under `==`.
            assert_eq!(get_f32_le(&mut cursor).to_bits(), value.to_bits());
        }
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.0f64] {
            let bytes = write::<8>(|c| put_f64_le(c, value), 8);
            let mut cursor: &[u8] = &bytes;
            assert_eq!(get_f64_le(&mut cursor).to_bits(), value.to_bits());
        }
    }
}
