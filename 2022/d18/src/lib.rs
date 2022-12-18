use glam::IVec3;
use std::collections::HashSet;

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

pub fn solution_2(_input: &str) -> usize {
    todo!()
}
