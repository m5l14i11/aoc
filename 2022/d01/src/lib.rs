use std::str::FromStr;

fn get_calories(input: &str) -> Vec<usize> {
    let mut result = Vec::new();

    for calories in input.split("\n\n") {
        let mut total = 0;
        
        for item in calories.lines() {
            total += usize::from_str(item).unwrap();
        }

        result.push(total);
    }
    result
}

pub fn solution_1(input: &str) -> usize {
    get_calories(input).iter().max().copied().unwrap()
}

pub fn solution_2(input: &str) -> usize {
    let mut result = get_calories(input);

    result.sort_by(|a, b| b.cmp(a));

    result.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn solution_1_test() {
        let result = solution_1(TEST_DATA.trim());
        assert_eq!(result, 24000);
    }

    #[test]
    fn solution_2_test() {
        let result = solution_2(TEST_DATA.trim());
        assert_eq!(result, 45000);
    }
}