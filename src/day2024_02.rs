use anyhow::*;
use itertools::Itertools;
use std::io::BufRead;
use std::iter;

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

pub fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let answer = reader
        .lines()
        .filter_ok(|line| {
            let chars: Vec<u8> = line.split(' ').map(|x| x.parse::<u8>().unwrap()).collect();
            is_safe(chars, usize::MAX)
        })
        .count();
    Ok(answer)
}

pub fn part2<R: BufRead>(reader: R) -> Result<usize> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_sample() {
        let res = part1(BufReader::new(TEST.as_bytes())).unwrap();
        assert_eq!(2, res);
    }

    #[test]
    fn part2_sample() {
        let res = part2(BufReader::new(TEST.as_bytes())).unwrap();
        assert_eq!(4, res);
    }
}
