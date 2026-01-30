use rand::{RngCore, SeedableRng};
use std::{
    fmt::LowerHex,
    io::{Cursor, Result},
};

pub(crate) fn init_test_vec(_size: usize, _seed: u32) -> Vec<u8> {
    b"1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_vec()
}

pub(crate) fn init_vec(size: usize, seed: u32) -> Vec<u8> {
    if size == 0 {
        return init_test_vec(size, seed);
    }
    let mut buffer = vec![0; size];
    let mut rng = rand::rngs::SmallRng::seed_from_u64(seed as u64);
    rng.fill_bytes(&mut buffer);
    buffer
}
pub(crate) fn init_cursor(size: usize, seed: u32) -> Cursor<Vec<u8>> {
    Cursor::new(init_vec(size, seed))
}

pub(crate) fn get_res_hash<T: LowerHex>(result: T) -> String {
    format!("{:x}", result)
}
pub(crate) fn get_res_result<T: LowerHex>(result: Result<T>) -> String {
    match result {
        Ok(res) => format!("{:x}", res),
        Err(err) => format!("{:?}", err),
    }
}
pub(crate) fn get_res_string(result: String) -> String {
    result
}
type CursorFn<T, S, R> = fn(&mut T, S) -> Result<R>;
type SliceFn<S, R> = fn(&[u8], S) -> R;

pub(crate) fn run_cursor<S: From<u32>, R>(mut data: Cursor<Vec<u8>>, seed: u32, mfn: CursorFn<Cursor<Vec<u8>>, S, R>) -> Result<R> {
    mfn(&mut data, seed.into())
}
pub(crate) fn run_slice<S: From<u32>, R>(data: Vec<u8>, seed: u32, mfn: SliceFn<S, R>) -> R {
    mfn(&data, seed.into())
}
