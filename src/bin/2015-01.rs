use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "2015-01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "(())";
const TEST2: &str = "()()";
const TEST3: &str = "(((";
const TEST4: &str = "(()(()(";
const TEST5: &str = "))(((((";
const TEST6: &str = "())";
const TEST7: &str = "))(";
const TEST8: &str = ")))";
const TEST9: &str = ")())())";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<isize> {
        let bytes = reader.bytes();
        let braces = bytes.counts_by(|x| x.unwrap() == b'(');
        let left_braces = braces
            .get(&true)
            .iter()
            .map(|&&i| i as isize)
            .sum::<isize>();
        let right_braces = braces
            .get(&false)
            .iter()
            .map(|&&i| i as isize)
            .sum::<isize>();
        Ok(left_braces - right_braces)
    }

    assert_eq!(0, part1(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(0, part1(BufReader::new(TEST2.as_bytes()))?);
    assert_eq!(3, part1(BufReader::new(TEST3.as_bytes()))?);
    assert_eq!(3, part1(BufReader::new(TEST4.as_bytes()))?);
    assert_eq!(3, part1(BufReader::new(TEST5.as_bytes()))?);
    assert_eq!(-1, part1(BufReader::new(TEST6.as_bytes()))?);
    assert_eq!(-1, part1(BufReader::new(TEST7.as_bytes()))?);
    assert_eq!(-3, part1(BufReader::new(TEST8.as_bytes()))?);
    assert_eq!(-3, part1(BufReader::new(TEST9.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    assert_eq!(74, result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
