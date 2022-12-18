use glam::IVec3;
use std::{
    cmp::{max, min},
    collections::HashSet,
};

fn get_cubes(input: &str) -> HashSet<IVec3> {
    input
        .lines()
        .map(|line| {
            let vec = line
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            IVec3::new(vec[0], vec[1], vec[2])
        })
        .collect::<HashSet<IVec3>>()
}

pub fn solution_1(input: &str) -> usize {
    let cubes = get_cubes(input);
    let adj_coords = vec![
        (0, 0, 1),
        (0, 1, 0),
        (1, 0, 0),
        (0, 0, -1),
        (0, -1, 0),
        (-1, 0, 0),
    ];

    cubes
        .iter()
        .map(|&IVec3 { x, y, z }| {
            adj_coords
                .iter()
                .map(|&(dx, dy, dz)| IVec3::new(x + dx, y + dy, z + dz))
                .filter(|x| cubes.get(x).is_none())
                .count()
        })
        .sum::<usize>()
}

fn exposed(
    pos: &IVec3,
    adj_coords: &Vec<(i32, i32, i32)>,
    cubes: &HashSet<IVec3>,
    min_coords: i32,
    max_coords: i32,
) -> bool {
    let mut stack = Vec::new();
    
    stack.push(pos.clone());

    let mut seen: HashSet<IVec3> = HashSet::new();

    if cubes.get(pos).is_some() {
        return false;
    }

    while stack.len() > 0 {
        let pop = stack.pop().unwrap();

        if cubes.get(&pop).is_some() {
            continue;
        }

        let IVec3{x, y, z} = pop;

        if !(min_coords <= x && x <= max_coords) {
            return true;
        }

        if !(min_coords <= y && y <= max_coords) {
            return true;
        }

        if !(min_coords <= z && z <= max_coords) {
            return true;
        }

        if seen.get(&pop).is_some() {
            continue;
        }

        seen.insert(pop);

        adj_coords.iter().for_each(|&(dx, dy, dz)| {
            stack.push(IVec3::new(pop.x + dx, pop.y + dy, pop.z + dz))
        });
    }

    false
}

pub fn solution_2(input: &str) -> usize {
    let mut min_coords = i32::MAX;
    let mut max_coords = i32::MIN;
    
    let cubes = get_cubes(input);

    cubes.iter().for_each(|vec| {
        min_coords = min(min_coords, vec.min_element());
        max_coords = max(max_coords, vec.max_element());
    });

    let adj_coords = vec![
        (0, 0, 1),
        (0, 1, 0),
        (1, 0, 0),
        (0, 0, -1),
        (0, -1, 0),
        (-1, 0, 0),
    ];

    cubes
        .iter()
        .map(|&IVec3 { x, y, z }| {
            adj_coords
                .iter()
                .map(|&(dx, dy, dz)| IVec3::new(x + dx, y + dy, z + dz))
                .filter(|pos| exposed(pos, &adj_coords, &cubes, min_coords, max_coords))
                .count()
        })
        .sum::<usize>()
}
