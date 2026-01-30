use crate::common::{get_res_hash, get_res_result, get_res_string, init_cursor, init_vec};
use digest::Digest;
use std::{
    fmt::{Display, LowerHex},
    io::{Cursor, Result},
    time::{Duration, Instant},
};

type InitFn<BufferT> = fn(usize, u32) -> BufferT;
type RunFn<BufferT, ResT> = fn(BufferT, u32) -> ResT;
type ResFn<ResT> = fn(ResT) -> String;
pub(crate) struct HashTest<BufferT, ResT> {
    name: String,
    size: usize,
    seed: u32,
    init_fn: InitFn<BufferT>,
    run_fn: RunFn<BufferT, ResT>,
    res_fn: ResFn<ResT>,
}
#[derive(Debug)]
pub struct Stat {
    pub name: String,
    pub size: usize,
    pub seed: u32,
    pub duration: Duration,
    pub res_string: String,
}
impl Stat {
    pub fn size_per_sec(&self) -> f64 {
        Duration::from_secs(1).div_duration_f64(self.duration) * self.size as f64
    }
    pub fn speed(&self) -> String {
        let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "YB", "BB"];
        let base = 1024f64;
        let mut speed = self.size_per_sec();
        let mut unit_index = 0;
        while unit_index + 1 < units.len() && speed > base {
            unit_index += 1;
            speed /= base;
        }
        format!("{:.2}{}/s", speed, units[unit_index])
    }
}
impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:<34} size: {:<8} duration: {:<14} speed: {:<12} seed: {:<8} res:{}",
            self.name,
            self.size,
            format!("{:?}", self.duration),
            self.speed(),
            self.seed,
            self.res_string
        ))
    }
}
pub(crate) trait HashTestRun {
    fn run(self) -> Stat;
}
impl<BufferT, ResT> HashTest<BufferT, ResT> {
    pub(crate) fn new(name: &str, size: usize, seed: u32, init_fn: InitFn<BufferT>, run_fn: RunFn<BufferT, ResT>, res_fn: ResFn<ResT>) -> Self {
        Self {
            name: name.to_string(),
            size,
            seed,
            init_fn,
            run_fn,
            res_fn,
        }
    }
}
impl<R: LowerHex> HashTest<Cursor<Vec<u8>>, Result<R>> {
    pub(crate) fn new_cursor(name: &str, size: usize, seed: u32, run_fn: RunFn<Cursor<Vec<u8>>, Result<R>>) -> Self {
        Self::new(name, size, seed, init_cursor, run_fn, get_res_result)
    }
}
impl<R: LowerHex> HashTest<Vec<u8>, R> {
    pub(crate) fn new_vec(name: &str, size: usize, seed: u32, run_fn: RunFn<Vec<u8>, R>) -> Self {
        Self::new(name, size, seed, init_vec, run_fn, get_res_hash)
    }
}
impl HashTest<Vec<u8>, String> {
    pub(crate) fn new_vec_str(name: &str, size: usize, seed: u32, run_fn: RunFn<Vec<u8>, String>) -> Self {
        Self::new(name, size, seed, init_vec, run_fn, get_res_string)
    }
}

impl HashTest<Vec<u8>, String> {
    pub(crate) fn new_wrapper<D: Digest>(name: &str, size: usize, seed: u32) -> Self {
        Self::new(name, size, seed, init_vec, |data, _: u32| hex::encode(D::digest(data)), get_res_string)
    }
}

impl<BufferT, ResT> HashTestRun for HashTest<BufferT, ResT> {
    fn run(self) -> Stat {
        let buffer = (self.init_fn)(self.size, self.seed);
        let start = Instant::now();
        let res = (self.run_fn)(buffer, self.seed);
        let duration = start.elapsed();
        let res_string = (self.res_fn)(res);
        Stat {
            name: self.name,
            size: self.size,
            seed: self.seed,
            duration,
            res_string,
        }
    }
}
