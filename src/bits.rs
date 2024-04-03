use core::mem;

fn put_chunk<const N: usize>(cursor: &mut &mut [u8], data: [u8; N]) {
    let buf = mem::replace(cursor, &mut []);
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
