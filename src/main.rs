use std::{
    fs::File,
    io::{Result, Write},
};

mod common;
mod hash_test;
mod renchmark;

fn test_value() {
    for stat in renchmark::btest(0, 0) {
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
        for stat in renchmark::btest(size, seed) {
            csv.write_fmt(format_args!(
                "{},{},{:?},{},{},{}\n",
                stat.name,
                stat.size,
                stat.duration,
                stat.duration.as_secs_f64(),
                stat.speed(),
                stat.size_per_sec()
            ))?;
            println!("{}", stat)
        }
    }
    Ok(())
}
