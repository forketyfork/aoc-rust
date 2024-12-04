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

    fn parse<R: BufRead>(reader: R) -> Vec<Vec<u8>> {
        reader
            .lines()
            .map(|line| {
                return line.unwrap().bytes().collect();
            })
            .collect()
    }

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
        let xmas: Vec<u8> = "XMAS".bytes().collect();
        for i in 0..array.len() {
            for j in 0..array[i].len() {
                if array[i][j] == 'X' as u8 {
                    'dir_loop: for dir in DIRECTIONS {
                        for idx in 1..=3 {
                            let ii = (i as i64) + dir.0 * idx;
                            let jj = (j as i64) + dir.1 * idx;
                            if ii < 0
                                || (ii as usize) >= array.len()
                                || jj < 0
                                || (jj as usize) >= array[ii as usize].len()
                                || array[ii as usize][jj as usize] != xmas[idx as usize]
                            {
                                continue 'dir_loop;
                            }
                        }
                        answer += 1;
                    }
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

    //noinspection SpellCheckingInspection
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let array = parse(reader);
        let mut answer = 0;
        for i in 0..array.len() - 2 {
            for j in 0..array[i].len() - 2 {
                let s: String = String::from_utf8(
                    [
                        array[i][j],
                        array[i][j + 2],
                        array[i + 1][j + 1],
                        array[i + 2][j],
                        array[i + 2][j + 2],
                    ]
                    .to_vec(),
                )?;

                if s == "MMASS" || s == "SMASM" || s == "MSAMS" || s == "SSAMM" {
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
