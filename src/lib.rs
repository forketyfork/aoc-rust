pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

use std::fs::File;
use std::io::BufReader;

/// Read the input file for a given day.
pub fn read_input(day: &str) -> anyhow::Result<BufReader<File>> {
    let path = format!("input/{}.txt", day);
    Ok(BufReader::new(File::open(path)?))
}

/// Parse a whitespace separated list of `u32` values.
pub fn parse_u32s(s: &str) -> Vec<u32> {
    s.split_whitespace()
        .filter_map(|v| v.parse::<u32>().ok())
        .collect()
}

pub mod day2015_01;
pub mod day2024_01;
pub mod day2024_02;
pub mod day2024_03;
pub mod day2024_04;
pub mod day2024_05;

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn it_works() {
        start_day("00");
    }

    proptest! {
        #[test]
        fn parse_round_trip(nums in proptest::collection::vec(0u32..1000, 0..10)) {
            let s = nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");
            let parsed = parse_u32s(&s);
            prop_assert_eq!(parsed, nums);
        }
    }
}
