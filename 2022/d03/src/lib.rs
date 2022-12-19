#![feature(iter_array_chunks)]

use std::collections::{HashMap, BTreeSet};

fn get_scores() -> HashMap<char, usize> {
    (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .enumerate()
        .map(|(idx, c)| (c as char, idx + 1))
        .collect()
}

pub fn solution_1(input: &str) -> usize {
    let scores = get_scores();

    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let a = a.chars().collect::<BTreeSet<char>>();
            let b = b.chars().collect::<BTreeSet<char>>();

            let common_char = a.intersection(&b).min().unwrap();

            *scores.get(common_char).unwrap_or(&0)
        })
        .sum()
}

pub fn solution_2(input: &str) -> usize {
    let scores = get_scores();

    input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let a_chars = a.chars().collect::<Vec<char>>();
            let b_chars = b.chars().collect::<Vec<char>>();
            let c_chars = c.chars().collect::<Vec<char>>();

            let common_char = a_chars
                .iter()
                .filter(|&c| b_chars.contains(c))
                .filter(|&c| c_chars.contains(c))
                .min_by_key(|c| *scores.get(c).unwrap_or(&0))
                .unwrap_or(&'\0');

            *scores.get(common_char).unwrap_or(&0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn solution_1_test() {
        let result = solution_1(TEST_DATA.trim());
        assert_eq!(result, 157);
    }

    #[test]
    fn solution_2_test() {
        let result = solution_2(TEST_DATA.trim());
        assert_eq!(result, 70);
    }
}