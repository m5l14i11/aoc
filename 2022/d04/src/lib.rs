use regex::Regex;
use std::cmp::{max, min};

fn parse_coordinates(line: &str) -> Option<(usize, usize, usize, usize)> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
   
    let captures = re.captures(line)?;
    let x0 = captures[1].parse::<usize>().ok()?;
    let y0 = captures[2].parse::<usize>().ok()?;
    let x1 = captures[3].parse::<usize>().ok()?;
    let y1 = captures[4].parse::<usize>().ok()?;

    Some((x0, y0, x1, y1))
}

pub fn solution_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (x0, y0, x1, y1) = parse_coordinates(line).unwrap_or((0, 0, 0, 0));

            match (x0 >= x1, y0 <= y1) {
                (true, true) | (false, false) => 1,
                _ => 0,
            }
        })
        .sum()
}

pub fn solution_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (x0, y0, x1, y1) = parse_coordinates(line).unwrap_or((0, 0, 0, 0));

            if max(x0, x1) <= min(y0, y1) {
                return 1;
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn solution_1_test() {
        let result = solution_1(TEST_DATA.trim());
        assert_eq!(result, 2);
    }

    #[test]
    fn solution_2_test() {
        let result = solution_2(TEST_DATA.trim());
        assert_eq!(result, 4);
    }
}