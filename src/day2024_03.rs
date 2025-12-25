use anyhow::*;
use std::io::BufRead;

pub fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let rx = regex::Regex::new(r"mul\((\d+),(\d+)\)")?;
    let answer = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let result: usize = rx
                .captures_iter(&line)
                .map(|it| {
                    let (_, [a, b]) = it.extract();
                    let a: usize = a.parse().unwrap();
                    let b: usize = b.parse().unwrap();
                    a * b
                })
                .sum();
            result
        })
        .sum();

    Ok(answer)
}

pub fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let rx = regex::Regex::new(r"(do)(\(\))|(don't)(\(\))|mul\((\d+),(\d+)\)")?;
    let mut do_command = true;
    let answer = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let result: usize = rx
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
                        return a * b;
                    }
                    0
                })
                .sum();
            result
        })
        .sum();
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_sample() {
        let res = part1(BufReader::new(TEST.as_bytes())).unwrap();
        assert_eq!(161, res);
    }

    #[test]
    fn part2_sample() {
        let res = part2(BufReader::new(TEST2.as_bytes())).unwrap();
        assert_eq!(48, res);
    }
}
