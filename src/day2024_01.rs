use anyhow::*;
use itertools::Itertools;
use std::io::BufRead;

fn parse_lists<R: BufRead>(reader: R) -> Result<(Vec<usize>, Vec<usize>), Error> {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];
    for line in reader.lines() {
        let line = line?;
        let pair: Vec<_> = line.split("   ").collect();
        let left_num: usize = pair[0].parse()?;
        let right_num: usize = pair[1].parse()?;
        left.push(left_num);
        right.push(right_num);
    }
    Ok((left, right))
}

pub fn part1<R: BufRead>(reader: R) -> Result<usize> {
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

pub fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let (left, right) = parse_lists(reader)?;
    let second_map = right.iter().counts_by(|c| c);
    let answer = left
        .iter()
        .map(|x| x * (second_map.get(x).unwrap_or(&0)))
        .sum::<usize>();
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_sample() {
        let res = part1(BufReader::new(TEST.as_bytes())).unwrap();
        assert_eq!(11, res);
    }

    #[test]
    fn part2_sample() {
        let res = part2(BufReader::new(TEST.as_bytes())).unwrap();
        assert_eq!(31, res);
    }
}
