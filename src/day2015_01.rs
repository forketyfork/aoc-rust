use anyhow::*;
use itertools::Itertools;
use std::io::BufRead;

pub fn part1<R: BufRead>(reader: R) -> Result<isize> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    const TEST1: &str = "(())";
    const TEST2: &str = "()()";
    const TEST3: &str = "(((";
    const TEST4: &str = "(()(()(";
    const TEST5: &str = "))(((((";
    const TEST6: &str = "())";
    const TEST7: &str = "))(";
    const TEST8: &str = ")))";
    const TEST9: &str = ")())())";

    #[test]
    fn part1_sample1() {
        let res = part1(BufReader::new(TEST1.as_bytes())).unwrap();
        assert_eq!(0, res);
    }

    #[test]
    fn part1_sample2() {
        let res = part1(BufReader::new(TEST2.as_bytes())).unwrap();
        assert_eq!(0, res);
    }

    #[test]
    fn part1_sample3() {
        let res = part1(BufReader::new(TEST3.as_bytes())).unwrap();
        assert_eq!(3, res);
    }

    #[test]
    fn part1_sample4() {
        let res = part1(BufReader::new(TEST4.as_bytes())).unwrap();
        assert_eq!(3, res);
    }

    #[test]
    fn part1_sample5() {
        let res = part1(BufReader::new(TEST5.as_bytes())).unwrap();
        assert_eq!(3, res);
    }

    #[test]
    fn part1_sample6() {
        let res = part1(BufReader::new(TEST6.as_bytes())).unwrap();
        assert_eq!(-1, res);
    }

    #[test]
    fn part1_sample7() {
        let res = part1(BufReader::new(TEST7.as_bytes())).unwrap();
        assert_eq!(-1, res);
    }

    #[test]
    fn part1_sample8() {
        let res = part1(BufReader::new(TEST8.as_bytes())).unwrap();
        assert_eq!(-3, res);
    }

    #[test]
    fn part1_sample9() {
        let res = part1(BufReader::new(TEST9.as_bytes())).unwrap();
        assert_eq!(-3, res);
    }
}
