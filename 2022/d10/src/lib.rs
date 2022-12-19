use itertools::Itertools;
use std::{collections::BTreeMap, fmt::Display, ops::RangeInclusive, str::FromStr};
use CPU::*;

const SIZE: usize = 40;
enum CPU {
    Noop,
    Add(i32),
}

impl CPU {
    fn cycles(&self) -> i32 {
        match self {
            Noop => 1,
            Add(_) => 2,
        }
    }
}

impl FromStr for CPU {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        match parts.next() {
            Some("noop") => Ok(CPU::Noop),
            Some("addx") => parts
                .next()
                .and_then(|num| num.parse::<i32>().ok())
                .map(CPU::Add)
                .ok_or("Error".to_string()),
            _ => Ok(CPU::Noop),
        }
    }
}

struct Computer {
    x: i32,
    cycles: i32,
    pixels: String,
}

impl Computer {
    fn new() -> Self {
        Computer {
            x: 1,
            cycles: 0,
            pixels: String::with_capacity(SIZE),
        }
    }

    fn sprite_range(&self) -> RangeInclusive<i32> {
        (self.x - 1)..=(self.x + 1)
    }

    fn interpret(&mut self, instruction: &CPU) {
        self.pixels.reserve(instruction.cycles() as usize);

        for _ in 0..instruction.cycles() {
            let cycle_guard = self.start_cycle();

            if cycle_guard
                .computer
                .sprite_range()
                .contains(&cycle_guard.pixel)
            {
                cycle_guard.computer.pixels.push_str("#");
            } else {
                cycle_guard.computer.pixels.push_str(".")
            }
        }

        match instruction {
            Noop => {}
            Add(num) => {
                self.x += num;
            }
        };
    }

    fn start_cycle(&mut self) -> Cycle {
        Cycle {
            pixel: self.cycles % SIZE as i32,
            computer: self,
        }
    }
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.pixels
                .chars()
                .chunks(SIZE)
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .join("\n")
        )
    }
}
struct Cycle<'a> {
    pixel: i32,
    computer: &'a mut Computer,
}

impl<'a> Drop for Cycle<'a> {
    fn drop(&mut self) {
        self.computer.cycles += 1;
    }
}

fn get_instructions(input: &str) -> Vec<CPU> {
    input
        .lines()
        .map(|line| line.parse::<CPU>().unwrap())
        .collect()
}

pub fn solution_1(input: &str) -> i32 {
    let instructions = get_instructions(input);
    let mut scores: BTreeMap<i32, i32> = BTreeMap::new();
    let cycles = [20, 60, 100, 140, 180, 220];
    let mut x: i32 = 1;
    let mut c: i32 = 0;

    for instruction in instructions {
        for cycle in cycles {
            if c + 1 == cycle {
                scores.insert(c + 1, (c + 1) * x);
            }

            if c + 2 == cycle {
                scores.insert(c + 2, (c + 2) * x);
            }
        }

        c += instruction.cycles();

        if let CPU::Add(num) = instruction {
            x += num;
        }
    }

    scores.values().sum()
}

pub fn solution_2(input: &str) -> String {
    let instructions = get_instructions(input);
    let mut computer = Computer::new();
    
    for instruction in instructions {
        computer.interpret(&instruction);
    }

    computer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn solution_1_test() {
        let result = solution_1(TEST_DATA.trim());
        assert_eq!(result, 13140);
    }

    #[test]
    fn solution_2_test() {
        let result = solution_2(TEST_DATA.trim());
        assert_eq!(
            result,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }

}