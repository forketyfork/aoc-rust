use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "2024-04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

//noinspection SpellCheckingInspection
const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> Vec<Vec<char>> {
        reader
            .lines()
            .map(|line| {
                return line.unwrap().chars().collect();
            })
            .collect()
    }

    const XMAS: &str = "XMAS";

    const DIRECTIONS: [(i64, i64); 8] = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let array = parse(reader);
        let mut answer = 0;
        for i in 0..array.len() {
            for j in 0..array[i].len() {
                'dir_loop: for dir in DIRECTIONS {
                    for idx in 0..=3 {
                        let ii = (i as i64) + dir.0 * idx;
                        let jj = (j as i64) + dir.1 * idx;
                        if ii < 0
                            || (ii as usize) >= array.len()
                            || jj < 0
                            || (jj as usize) >= array[ii as usize].len()
                            || array[ii as usize][jj as usize]
                                != XMAS.chars().nth(idx as usize).unwrap()
                        {
                            continue 'dir_loop;
                        }
                    }
                    answer += 1;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    assert_eq!(2370, result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let array = parse(reader);
        let mut answer = 0;
        for i in 0..array.len() - 2 {
            for j in 0..array[i].len() - 2 {
                let s: String = [
                    array[i][j],
                    '.',
                    array[i][j + 2],
                    '.',
                    array[i + 1][j + 1],
                    '.',
                    array[i + 2][j],
                    '.',
                    array[i + 2][j + 2],
                ]
                .iter()
                .collect();

                if s == "M.M.A.S.S" || s == "S.M.A.S.M" || s == "M.S.A.M.S" || s == "S.S.A.M.M" {
                    answer += 1;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    assert_eq!(1908, result);
    // endregion

    Ok(())
}
