use anyhow::*;
use std::io::BufRead;

/// Parse the grid of letters from a reader.
fn parse<R: BufRead>(reader: R) -> Vec<Vec<u8>> {
    reader
        .lines()
        .map(|line| line.unwrap().bytes().collect())
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

/// Count occurrences of "XMAS" in any direction.
pub fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let array = parse(reader);
    let mut answer = 0;
    let xmas: Vec<u8> = "XMAS".bytes().collect();
    for i in 0..array.len() {
        for j in 0..array[i].len() {
            if array[i][j] == xmas[0] {
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

/// Count five-letter star patterns.
pub fn part2<R: BufRead>(reader: R) -> Result<usize> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

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

    #[test]
    fn part1_sample() {
        let res = part1(BufReader::new(TEST.as_bytes())).unwrap();
        assert_eq!(18, res);
    }

    #[test]
    fn part2_sample() {
        let res = part2(BufReader::new(TEST.as_bytes())).unwrap();
        assert_eq!(9, res);
    }
}
