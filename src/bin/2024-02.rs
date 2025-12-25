use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

const DAY: &str = "2024-02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn is_safe(chars: Vec<u8>, skipping: usize) -> bool {
        let mut increasing = 0;
        let mut prev = 0;
        for (idx, &char) in chars.iter().enumerate() {
            if idx == skipping {
                continue;
            }
            if prev == 0 {
                prev = char;
                continue;
            }
            if increasing == 0 {
                if char > prev {
                    increasing = 1;
                } else {
                    increasing = 2;
                }
            }
            if (increasing == 1 && char < prev || increasing == 2 && char > prev)
                || prev.abs_diff(char) > 3
                || prev.abs_diff(char) < 1
            {
                return false;
            }
            prev = char;
        }
        true
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .filter_ok(|line| {
                let chars: Vec<u8> = line.split(' ').map(|x| x.parse::<u8>().unwrap()).collect();
                is_safe(chars, usize::MAX)
            })
            .count();
        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    assert_eq!(510, result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .filter_ok(|line| {
                let chars: Vec<u8> = line.split(' ').map(|x| x.parse::<u8>().unwrap()).collect();
                iter::once(usize::MAX)
                    .chain(0..chars.len())
                    .any(|skip_index| is_safe(chars.clone(), skip_index))
            })
            .count();
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    assert_eq!(553, result);
    // endregion

    Ok(())
}
