use std::{
    fs::File,
    io::{Result, Write},
};

mod common;
mod hash_test;
mod brenchmark;

fn test_value() {
    for stat in brenchmark::btest(0, 0x12345678) {
        println!("{}", stat)
    }
}
fn main() -> Result<()> {
    test_value();
    let mut csv = File::create("stat.csv")?;
    csv.write_fmt(format_args!("name,size,duration,duration(s),speed,size_per_sec\n"))?;
    let seed = 0;
    for bit in 0..25 {
        let size = 1 << bit;
        for stat in brenchmark::btest(size, seed) {
            csv.write_fmt(format_args!(
                "{},{},{:?},{},{},{}\n",
                stat.name,
                stat.size,
                stat.duration,
                stat.duration.as_secs_f64(),
                stat.speed(),
                stat.size_per_sec()
            ))?;
        }
    }
    Ok(())
}
