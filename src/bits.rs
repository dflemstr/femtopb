//! Fixed-width little-endian reads and writes over a cursor into a borrowed buffer.

use core::mem;

/// Writes `data` at the cursor and advances past it.
///
/// If fewer than `N` bytes remain, the write is skipped and the cursor is left empty rather than
/// panicking: `Message::encode` sizes the buffer from `encoded_len` first, so this is unreachable
/// through the checked API, and staying panic-free is what lets the encoding path carry the crate's
/// no-panic guarantee.
fn put_chunk<const N: usize>(cursor: &mut &mut [u8], data: [u8; N]) {
    let buf = mem::take(cursor);
    let Some((chunk, rest)) = buf.split_first_chunk_mut::<N>() else {
        return;
    };
    *chunk = data;
    *cursor = rest;
}

/// Reads `N` bytes at the cursor and advances past them, or returns `None` if fewer remain.
///
/// Reporting the shortfall rather than panicking is what lets the `fixed_width!` runtime's
/// `decode_single_value` turn it into a `BufferUnderflow` error, and keeps the bounds check
/// provable instead of relying on a separate length test the optimizer has to match up.
fn take_chunk<const N: usize>(cursor: &mut &[u8]) -> Option<[u8; N]> {
    let (&chunk, rest) = cursor.split_first_chunk()?;
    *cursor = rest;
    Some(chunk)
}

pub fn put_f32_le(cursor: &mut &mut [u8], value: f32) {
    put_chunk(cursor, value.to_le_bytes());
}

pub fn get_f32_le(cursor: &mut &[u8]) -> Option<f32> {
    Some(f32::from_le_bytes(take_chunk(cursor)?))
}

pub fn put_f64_le(cursor: &mut &mut [u8], value: f64) {
    put_chunk(cursor, value.to_le_bytes());
}

pub fn get_f64_le(cursor: &mut &[u8]) -> Option<f64> {
    Some(f64::from_le_bytes(take_chunk(cursor)?))
}

pub fn put_u32_le(cursor: &mut &mut [u8], value: u32) {
    put_chunk(cursor, value.to_le_bytes());
}

pub fn get_u32_le(cursor: &mut &[u8]) -> Option<u32> {
    Some(u32::from_le_bytes(take_chunk(cursor)?))
}

pub fn put_u64_le(cursor: &mut &mut [u8], value: u64) {
    put_chunk(cursor, value.to_le_bytes());
}

pub fn get_u64_le(cursor: &mut &[u8]) -> Option<u64> {
    Some(u64::from_le_bytes(take_chunk(cursor)?))
}

pub fn put_i32_le(cursor: &mut &mut [u8], value: i32) {
    put_chunk(cursor, value.to_le_bytes());
}

pub fn get_i32_le(cursor: &mut &[u8]) -> Option<i32> {
    Some(i32::from_le_bytes(take_chunk(cursor)?))
}

pub fn put_i64_le(cursor: &mut &mut [u8], value: i64) {
    put_chunk(cursor, value.to_le_bytes());
}

pub fn get_i64_le(cursor: &mut &[u8]) -> Option<i64> {
    Some(i64::from_le_bytes(take_chunk(cursor)?))
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
                assert_eq!($get(&mut cursor), Some($val));
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
        assert_eq!(get_u32_le(&mut cursor), Some(1));
        assert_eq!(cursor, &[0xAA, 0xBB]); // trailing bytes untouched, cursor advanced by 4
    }

    #[test]
    fn get_reports_a_short_buffer_instead_of_panicking() {
        // Fewer bytes than the value is wide: `None`, and the cursor is left where it was so the
        // caller can report the shortfall against the original input.
        for short in [&[][..], &[1][..], &[1, 2, 3][..]] {
            let mut cursor: &[u8] = short;
            assert_eq!(get_u32_le(&mut cursor), None, "input {short:?}");
            assert_eq!(cursor, short, "a failed read must not consume anything");
        }
        let mut cursor: &[u8] = &[1, 2, 3, 4, 5, 6, 7];
        assert_eq!(get_u64_le(&mut cursor), None);
    }

    #[test]
    fn put_into_a_short_buffer_is_a_no_op_instead_of_a_panic() {
        // `Message::encode` sizes the buffer from `encoded_len` first, so this cannot happen
        // through the checked API; going through `encode_raw` directly must truncate, not panic.
        let mut buf = [0u8; 3];
        let mut cursor: &mut [u8] = &mut buf;
        put_u32_le(&mut cursor, 0xDEAD_BEEF);
        assert!(cursor.is_empty(), "an impossible write must exhaust the cursor");
        assert_eq!(buf, [0, 0, 0], "nothing may be written past the end");

        let mut empty: &mut [u8] = &mut [];
        put_f64_le(&mut empty, 1.0);
        assert!(empty.is_empty());
    }

    #[test]
    fn float_special_values_are_bit_preserved() {
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -0.0f32] {
            let bytes = write::<4>(|c| put_f32_le(c, value), 4);
            let mut cursor: &[u8] = &bytes;
            // Compare bit patterns: NaN != NaN and -0.0 == 0.0 under `==`.
            assert_eq!(
                get_f32_le(&mut cursor).map(f32::to_bits),
                Some(value.to_bits())
            );
        }
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.0f64] {
            let bytes = write::<8>(|c| put_f64_le(c, value), 8);
            let mut cursor: &[u8] = &bytes;
            assert_eq!(
                get_f64_le(&mut cursor).map(f64::to_bits),
                Some(value.to_bits())
            );
        }
    }
}
