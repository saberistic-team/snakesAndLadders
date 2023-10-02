use crate::*;

pub fn rand_range(min: u64, max: u64,seed:u32) -> Result<u64> {
    let slot = Clock::get()?.unix_timestamp;
    let seed = slot as u64 ^ seed as u64;
    let xorshift_output = xorshift(seed);
    let random_number = min + (xorshift_output % (max - min + 1));
    Ok(random_number)
}
fn xorshift(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 43;
    x as u64
}
