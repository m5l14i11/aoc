use std::cmp::Ordering;
use std::str::FromStr;

#[derive(PartialEq, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err("Illegal Move".to_string()),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Hand::*;
        match (self, other) {
            (Rock, Scissors) => Some(Ordering::Greater),
            (Scissors, Rock) => Some(Ordering::Less),
            _ => Some((*self as usize).cmp(&(*other as usize)))
        }
    }
}

impl Hand {
    fn score(self, num: usize) -> usize {
        self as usize + num
    }
}

pub fn solution_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let hands = line.split(" ").collect::<Vec<&str>>();

            let hand1 = hands[0].parse::<Hand>().unwrap();
            let hand2 = hands[1].parse::<Hand>().unwrap();

            match hand1.partial_cmp(&hand2) {
                Some(Ordering::Equal) => hand2.score(3),
                Some(Ordering::Less) => hand2.score(6),
                Some(Ordering::Greater) => hand2.score(0),
                None => 0,
            }
        })
        .sum()
}

pub fn solution_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let hands = line.split(" ").collect::<Vec<&str>>();

            let hand1 = hands[0].parse::<Hand>().unwrap();
            let hand2 = hands[1];

            let score = match hand2 {
                "Y" => 3,
                "Z" => 6,
                _ => 0,
            };

            let our_hand = match hand2 {
                "X" => match hand1 {
                    Hand::Rock => Hand::Scissors,
                    Hand::Paper => Hand::Rock,
                    Hand::Scissors => Hand::Paper,
                },
                "Y" => hand1,
                "Z" => match hand1 {
                    Hand::Rock => Hand::Paper,
                    Hand::Paper => Hand::Scissors,
                    Hand::Scissors => Hand::Rock,
                },
                _ => return 0,
            };

            our_hand.score(score)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "A Y
B X
C Z";

    #[test]
    fn solution_1_test() {
        let result = solution_1(TEST_DATA.trim());
        assert_eq!(result, 15);
    }

    #[test]
    fn solution_2_test() {
        let result = solution_2(TEST_DATA.trim());
        assert_eq!(result, 12);
    }
}