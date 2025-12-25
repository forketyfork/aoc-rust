use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "2024-05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut ordering: Vec<(u8, u8)> = vec![];
        let mut lines: Vec<Vec<u8>> = vec![];
        let mut first_part: bool = true;
        reader.lines().for_each(|line| {
            let line = line.unwrap();
            if line.trim().is_empty() {
                first_part = false;
            } else if first_part {
                let (left, right) = line.split("|").collect_tuple().unwrap();
                let left: u8 = left.parse().unwrap();
                let right: u8 = right.trim().parse().unwrap();
                ordering.push((left, right));
            } else {
                let v: Vec<u8> = line
                    .split(",")
                    .map(|number| {
                        let number: u8 = number.parse().unwrap();
                        number
                    })
                    .collect();
                lines.push(v);
            }
        });

        let ordering: HashMap<u8, Vec<u8>> = ordering.iter().copied().into_group_map();

        let answer = lines
            .iter()
            .filter_map(|line| {
                if pass(line, &ordering) {
                    line.get(line.len() / 2).map(|&x| x as usize)
                } else {
                    None
                }
            })
            .sum();

        Ok(answer)
    }
    fn pass(line: &Vec<u8>, ordering: &HashMap<u8, Vec<u8>>) -> bool {
        let mut seen: HashSet<u8> = HashSet::new();
        for num in line {
            seen.insert(*num);
            if ordering.contains_key(num) && ordering[num].iter().any(|after| seen.contains(after))
            {
                return false;
            }
        }
        true
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    assert_eq!(4662, result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut ordering: HashSet<String> = HashSet::new();
        let mut lines: Vec<Vec<String>> = vec![];
        let mut first_part: bool = true;
        reader.lines().for_each(|line| {
            let line = line.unwrap();
            if line.trim().is_empty() {
                first_part = false;
            } else if first_part {
                ordering.insert(line);
            } else {
                let v: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
                lines.push(v);
            }
        });

        let mut answer: usize = 0;
        lines.iter().for_each(|line| {
            let sorted_line: Vec<String> = line
                .iter()
                .sorted_by(|a, b| {
                    if ordering.contains(format!("{}|{}", a, b).as_str()) {
                        Ordering::Greater
                    } else if ordering.contains(format!("{}|{}", b, a).as_str()) {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                })
                .cloned()
                .collect();
            if sorted_line != *line {
                answer += sorted_line
                    .get(line.len() / 2)
                    .map(|x| x.parse::<usize>())
                    .unwrap()
                    .unwrap();
            }
        });

        Ok(answer)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    assert_eq!(0, result);
    //endregion

    Ok(())
}
