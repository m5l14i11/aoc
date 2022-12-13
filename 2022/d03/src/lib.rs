#![feature(iter_array_chunks)]

use std::collections::{BTreeMap, BTreeSet};

fn get_scores() -> BTreeMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<BTreeMap<char, usize>>()
}

pub fn solution_1(input: &str) -> usize {
    let scores = get_scores();

    input
        .lines()
        .map(|line| {
            let len = line.len();
            let a = &line[0..len / 2].chars().collect::<BTreeSet<_>>();
            let b = &line[len / 2..len].chars().collect();

            let common_char = a.intersection(&b).next().unwrap();

            scores.get(common_char).unwrap()
        })
        .sum()
}

pub fn solution_2(input: &str) -> usize {
    let scores = get_scores();

    input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let a_set = a.chars().collect::<BTreeSet<_>>();
            let b_set = b.chars().collect();
            let c_set = c.chars().collect();

            let a_and_b = a_set.intersection(&b_set).cloned().collect::<BTreeSet<_>>();

            let common_char = a_and_b.intersection(&c_set).next().unwrap();

            scores.get(&common_char).unwrap()
        })
        .sum()
}
