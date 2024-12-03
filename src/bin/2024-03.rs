use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "2024-03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

//noinspection SpellCheckingInspection
const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
//noinspection SpellCheckingInspection
const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let rx = regex::Regex::new(r"mul\((\d+),(\d+)\)")?;
        let answer: usize = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let result = rx
                    .captures_iter(&line)
                    .map(|it| {
                        let (_, [a, b]) = it.extract();
                        let a: usize = a.parse().unwrap();
                        let b: usize = b.parse().unwrap();
                        a * b
                    })
                    .sum::<usize>();
                result
            })
            .sum();

        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    assert_eq!(178538786, result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let rx = regex::Regex::new(r"(do)(\(\))|(don't)(\(\))|mul\((\d+),(\d+)\)")?;
        let mut do_command = true;
        let answer: usize = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let result = rx
                    .captures_iter(&line)
                    .map(|it| {
                        let (_, [a, b]) = it.extract();
                        if a == "do" {
                            do_command = true;
                        } else if a == "don't" {
                            do_command = false;
                        } else if do_command {
                            let a: usize = a.parse().unwrap();
                            let b: usize = b.parse().unwrap();
                            return a * b
                        }
                        0
                    })
                    .sum::<usize>();
                result
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    assert_eq!(102467299, result);
    // endregion

    Ok(())
}
