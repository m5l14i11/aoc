use glam::IVec3;
use std::{
    cmp::{max, min},
    collections::HashSet,
};

fn get_adjacent_coords() -> Vec<IVec3> {
    vec![
        IVec3::new(0, 0, 1),
        IVec3::new(0, 1, 0),
        IVec3::new(1, 0, 0),
        IVec3::new(0, 0, -1),
        IVec3::new(0, -1, 0),
        IVec3::new(-1, 0, 0),
    ]
}

fn parse_cubes(input: &str) -> HashSet<IVec3> {
    input
        .lines()
        .map(|line| {
            let vec = line
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            IVec3::from_array(vec.try_into().unwrap())
        })
        .collect()
}

fn is_exposed(pos: IVec3, min_max: (i32, i32), cubes: &HashSet<IVec3>) -> bool {
    let (min_coords, max_coords) = min_max;
    let mut stack = Vec::new();
    let mut seen = HashSet::new();

    stack.push(pos);

    while let Some(pop) = stack.pop() {
        if cubes.contains(&pop) {
            continue;
        }

        for coords in pop.to_array() {
            if !(min_coords <= coords && coords <= max_coords) {
                return true;
            }
        }

        if !seen.contains(&pop) {
            seen.insert(pop);

            for offset in get_adjacent_coords().iter() {
                stack.push(pop + *offset)
            }
        }
    }

    false
}

pub fn solution_1(input: &str) -> usize {
    let cubes = parse_cubes(input);

    cubes
        .iter()
        .map(|pos| {
            get_adjacent_coords()
                .iter()
                .map(|offset| *pos + *offset)
                .filter(|pos| !cubes.contains(pos))
                .count()
        })
        .sum()
}

pub fn solution_2(input: &str) -> usize {
    let mut min_coords = i32::MAX;
    let mut max_coords = i32::MIN;

    let cubes = parse_cubes(input);

    cubes.iter().for_each(|vec| {
        min_coords = min(min_coords, vec.min_element());
        max_coords = max(max_coords, vec.max_element());
    });

    cubes
        .iter()
        .map(|pos| {
            get_adjacent_coords()
                .iter()
                .map(|offset| *pos + *offset)
                .filter(|pos| is_exposed(*pos, (min_coords, max_coords), &cubes))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn solution_1_test() {
        let result = solution_1(TEST_DATA.trim());
        assert_eq!(result, 64);
    }

    #[test]
    fn solution_2_test() {
        let result = solution_2(TEST_DATA.trim());
        assert_eq!(result, 58);
    }
}
