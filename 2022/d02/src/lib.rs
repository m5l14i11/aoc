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
        if self == &Hand::Scissors && other == &Hand::Rock {
            Some(Ordering::Less);
        }

        if self == &Hand::Rock && other == &Hand::Scissors {
            Some(Ordering::Greater);
        }

        Some((*self as usize).cmp(&(*other as usize)))
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
            let hands = line
                .split(" ")
                .map(|item| item.parse::<Hand>().unwrap())
                .collect::<Vec<Hand>>();

            let hand1 = hands[0];
            let hand2 = hands[1];

            match hand1.partial_cmp(&hand2) {
                Some(Ordering::Equal) => hand2.score(3),
                Some(Ordering::Less) => hand2.score(6),
                Some(Ordering::Greater) => hand2.score(0),
                None => 0,
            }
        })
        .sum::<usize>()
}

pub fn solution_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let hands = line.split(" ").collect::<Vec<&str>>();

            let hand1 = hands.first().unwrap().parse::<Hand>().unwrap();
            let hand2 = hands[1];

            match hand2 {
                "X" => {
                    let our_hand = match hand1 {
                        Hand::Rock => Hand::Scissors,
                        Hand::Paper => Hand::Rock,
                        Hand::Scissors => Hand::Paper,
                    };
                    our_hand.score(0)
                }
                "Y" => hand1.score(3),
                "Z" => {
                    let our_hand = match hand1 {
                        Hand::Rock => Hand::Paper,
                        Hand::Paper => Hand::Scissors,
                        Hand::Scissors => Hand::Rock,
                    };
                    our_hand.score(6)
                }
                _ => 0,
            }
        })
        .sum::<usize>()
}
