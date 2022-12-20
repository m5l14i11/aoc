fn parse_numbers(input: &str) -> Vec<(usize, i64)> {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| (id, line.parse::<i64>().unwrap()))
        .collect::<Vec<_>>()
}

pub fn solution_1(input: &str) -> i64 {
    let numbers = parse_numbers(input);
    let mut state: Vec<(usize, i64)> = numbers.clone();

    for (id, _) in numbers {
        let index = state
            .iter()
            .position(|(state_id, _)| *state_id == id)
            .unwrap();

        let current = state.remove(index);
        let added = index as i64 + current.1;
        let new_index = added.rem_euclid(state.len() as i64);

        state.insert(new_index as usize, current);
    }

    let zero_pos = state
        .iter()
        .position(|(_, value)| value == &0)
        .unwrap();
    let a = state[(1000 + zero_pos) % state.len()].1;
    let b = state[(2000 + zero_pos) % state.len()].1;
    let c = state[(3000 + zero_pos) % state.len()].1;

    a + b + c
}

pub fn solution_2(input: &str) -> i64 {
    let mut numbers = parse_numbers(input);

    numbers
        .iter_mut()
        .for_each(|(_, value)| *value *= 811589153);

    let mut state = numbers.clone();

    for _ in 0..10 {
        for (id, _) in &numbers {
            let index = state
                .iter()
                .position(|(state_id, _)| state_id == id)
                .unwrap();

            let current = state.remove(index);
            let added = index as i64 + current.1;
            let new_index = added.rem_euclid(state.len() as i64);
        
            state.insert(new_index as usize, current);
        }
    }

    let zero_pos = state
        .iter()
        .position(|(_, value)| value == &0)
        .unwrap();
    
    let a = state[(1000 + zero_pos) % state.len()].1;
    let b = state[(2000 + zero_pos) % state.len()].1;
    let c = state[(3000 + zero_pos) % state.len()].1;

    a + b + c
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn solution_1_test() {
        let result = solution_1(TEST_DATA.trim());
        assert_eq!(result, 3);
    }

    #[test]
    fn solution_2_test() {
        let result = solution_2(TEST_DATA.trim());
        assert_eq!(result, 1623178306);
    }
}