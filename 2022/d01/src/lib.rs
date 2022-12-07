fn get_elves(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|item| item.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect()
}

pub fn solution_1(input: &str) -> usize {
    get_elves(input).into_iter().max().unwrap()
}

pub fn solution_2(input: &str) -> usize {
    let mut result = get_elves(input);

    result.sort_by(|a, b| b.cmp(a));

    result.iter().take(3).sum()
}
