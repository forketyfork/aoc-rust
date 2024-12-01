use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "2024-01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

fn main() -> Result<()> {
    start_day(DAY);

    fn parse_lists<R: BufRead>(reader: R) -> Result<(Vec<usize>, Vec<usize>), Error> {
        let mut left: Vec<usize> = vec![];
        let mut right: Vec<usize> = vec![];
        for line in reader.lines() {
            let line = line?;
            let pair: Vec<_> = line.split("   ").collect();
            let left_num = pair[0].parse::<usize>();
            let right_num = pair[1].parse::<usize>();
            left.push(left_num?);
            right.push(right_num?);
        }
        Ok((left, right))
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut left, mut right) = parse_lists(reader)?;
        left.sort();
        right.sort();
        let answer = left
            .iter()
            .zip(right.iter())
            .map(|(x, y)| x.abs_diff(*y))
            .sum();
        Ok(answer)
    }
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    assert_eq!(2264607, result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (left, right) = parse_lists(reader)?;
        let second_map = right.iter().counts_by(|c| c);
        let answer = left
            .iter()
            .map(|x| x * (second_map.get(x).unwrap_or(&0)))
            .sum::<usize>();
        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    assert_eq!(19457120, result);
    //endregion

    Ok(())
}
