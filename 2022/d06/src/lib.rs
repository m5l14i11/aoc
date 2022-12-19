use std::collections::{HashSet};

fn process_characters(input: &str, window_size: usize) -> usize {
    let mut window_start = 0;

    while window_start + window_size <= input.len() {
        let set = input[window_start..window_start + window_size]
            .chars()
            .collect::<HashSet<char>>();
        
        if set.len() == window_size {
            return window_start + window_size;
        }
        
        window_start += 1;
    }

    0
}

pub fn solution_1(input: &str) -> usize {
    process_characters(input, 4)
}

pub fn solution_2(input: &str) -> usize {
    process_characters(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_test() {
        assert_eq!(solution_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solution_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solution_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solution_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solution_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn solution_2_test() {
        assert_eq!(solution_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solution_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solution_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solution_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solution_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
