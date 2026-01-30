use crc::{CRC_16_ARC, CRC_32_ISO_HDLC, CRC_64_REDIS, Crc};
use crc16::{ARC, State as Crc16State};
use crc32fast::hash as crc32fast_hash;
use crc64::crc64;
use murmur3::{murmur3_32, murmur3_32_of_slice, murmur3_x64_128, murmur3_x64_128_of_slice, murmur3_x86_128, murmur3_x86_128_of_slice};
use seahash::hash as seahash;
use sha256::digest as sha256_digest;
use twox_hash::{XxHash3_64, XxHash3_128, XxHash32, XxHash64};
use xxhash_rust::{
    const_xxh3::{xxh3_64_with_seed as const_xxh3_64_with_seed, xxh3_128_with_seed as const_xxh3_128_with_seed},
    const_xxh32::xxh32 as const_xxh32,
    const_xxh64::xxh64 as const_xxh64,
    xxh3::{xxh3_64_with_seed, xxh3_128_with_seed},
    xxh32::xxh32,
    xxh64::xxh64,
};

use crate::common::{run_cursor, run_slice};
use crate::hash_test::{HashTest, HashTestRun, Stat};
use blake2::{Blake2b512, Blake2s256};
use md5::compute as md5_compute;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Keccak224, Keccak256, Keccak256Full, Keccak384, Keccak512, Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use sm3::Sm3;

pub(crate) fn btest(size: usize, seed: u32) -> Vec<Stat> {
    vec![
        HashTest::new_vec("md5_compute", size, seed, |data, seed| run_slice(data, seed, |d, _: u32| md5_compute(d))).run(),
        HashTest::new_wrapper::<Blake2b512>("Blake2b512", size, seed).run(),
        HashTest::new_wrapper::<Blake2s256>("Blake2s256", size, seed).run(),
        HashTest::new_wrapper::<Sha1>("Sha1", size, seed).run(),
        HashTest::new_wrapper::<Keccak224>("Keccak224", size, seed).run(),
        HashTest::new_wrapper::<Keccak256>("Keccak256", size, seed).run(),
        HashTest::new_wrapper::<Keccak384>("Keccak384", size, seed).run(),
        HashTest::new_wrapper::<Keccak512>("Keccak512", size, seed).run(),
        HashTest::new_wrapper::<Keccak256Full>("Keccak256Full", size, seed).run(),
        HashTest::new_wrapper::<Sha3_224>("Sha3_224", size, seed).run(),
        HashTest::new_wrapper::<Sha3_256>("Sha3_256", size, seed).run(),
        HashTest::new_wrapper::<Sha3_384>("Sha3_384", size, seed).run(),
        HashTest::new_wrapper::<Sha3_512>("Sha3_512", size, seed).run(),
        HashTest::new_wrapper::<Sha224>("Sha224", size, seed).run(),
        HashTest::new_wrapper::<Sha256>("Sha256", size, seed).run(),
        HashTest::new_wrapper::<Sha384>("Sha384", size, seed).run(),
        HashTest::new_wrapper::<Sha512>("Sha512", size, seed).run(),
        HashTest::new_wrapper::<Sha512_224>("Sha512_224", size, seed).run(),
        HashTest::new_wrapper::<Sha512_256>("Sha512_256", size, seed).run(),
        HashTest::new_wrapper::<Sm3>("sm3", size, seed).run(),
        HashTest::new_vec("crc<u16>", size, seed, |data, seed| run_slice(data, seed, |d, _: u32| Crc::<u16>::new(&CRC_16_ARC).checksum(d))).run(),
        HashTest::new_vec("crc<u32>", size, seed, |data, seed| run_slice(data, seed, |d, _: u32| Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(d))).run(),
        HashTest::new_vec("crc<u64>", size, seed, |data, seed| run_slice(data, seed, |d, _: u32| Crc::<u64>::new(&CRC_64_REDIS).checksum(d))).run(),
        HashTest::new_vec("crc16", size, seed, |data, seed| run_slice(data, seed, |d, _: u32| Crc16State::<ARC>::calculate(d))).run(),
        HashTest::new_vec("crc32fast", size, seed, |data, seed| run_slice(data, seed, |d, _: u32| crc32fast_hash(d))).run(),
        HashTest::new_vec("crc64", size, seed, |data, seed| run_slice(data, seed, |d, _: u32| crc64(0, d))).run(),
        HashTest::new_vec_str("sha256", size, seed, |data, seed| run_slice(data, seed, |d, _: u32| sha256_digest(d))).run(),
        HashTest::new_vec("twox_hash::XxHash32", size, seed, |data, seed| run_slice(data, seed, |d, s| XxHash32::oneshot(s, d))).run(),
        HashTest::new_vec("twox_hash::XxHash64", size, seed, |data, seed| run_slice(data, seed, |d, s| XxHash64::oneshot(s, d))).run(),
        HashTest::new_vec("twox_hash::XxHash3_64", size, seed, |data, seed| run_slice(data, seed, |d, s: u32| XxHash3_64::oneshot_with_seed(s as u64, d))).run(),
        HashTest::new_vec("twox_hash::XxHash3_128", size, seed, |data, seed| run_slice(data, seed, |d, s: u32| XxHash3_128::oneshot_with_seed(s as u64, d))).run(),
        HashTest::new_vec("seahash", size, seed, |data, seed| run_slice(data, seed, |d, _: u32| seahash(d))).run(),
        HashTest::new_vec("xxhash::xxh32", size, seed, |data, seed| run_slice(data, seed, xxh32)).run(),
        HashTest::new_vec("xxhash::xxh64", size, seed, |data, seed| run_slice(data, seed, xxh64)).run(),
        HashTest::new_vec("xxhash::xxh3_64_with_seed", size, seed, |data, seed| run_slice(data, seed, xxh3_64_with_seed)).run(),
        HashTest::new_vec("xxhash::xxh3_128_with_seed", size, seed, |data, seed| run_slice(data, seed, xxh3_128_with_seed)).run(),
        HashTest::new_vec("xxhash::const_xxh32", size, seed, |data, seed| run_slice(data, seed, const_xxh32)).run(),
        HashTest::new_vec("xxhash::const_xxh64", size, seed, |data, seed| run_slice(data, seed, const_xxh64)).run(),
        HashTest::new_vec("xxhash::const_xxh3_64_with_seed", size, seed, |data, seed| run_slice(data, seed, const_xxh3_64_with_seed)).run(),
        HashTest::new_vec("xxhash::const_xxh3_128_with_seed", size, seed, |data, seed| run_slice(data, seed, const_xxh3_128_with_seed)).run(),
        HashTest::new_cursor("murmur3_32", size, seed, |data, seed| run_cursor(data, seed, murmur3_32)).run(),
        HashTest::new_cursor("murmur3_x64_128", size, seed, |data, seed| run_cursor(data, seed, murmur3_x64_128)).run(),
        HashTest::new_cursor("murmur3_x86_128", size, seed, |data, seed| run_cursor(data, seed, murmur3_x86_128)).run(),
        HashTest::new_vec("murmur3_32_of_slice", size, seed, |data, seed| run_slice(data, seed, murmur3_32_of_slice)).run(),
        HashTest::new_vec("murmur3_x64_128_of_slice", size, seed, |data, seed| run_slice(data, seed, murmur3_x64_128_of_slice)).run(),
        HashTest::new_vec("murmur3_x86_128_of_slice", size, seed, |data, seed| run_slice(data, seed, murmur3_x86_128_of_slice)).run(),
    ]
}
