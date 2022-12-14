use std::cmp::{max, min};

fn get_coords(line: &str) -> Vec<Vec<usize>> {
    let d = line.split(',').collect::<Vec<_>>();
    let x0y0 = d[0]
        .split('-')
        .map(|val| val.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let x1y1 = d[1]
        .split('-')
        .map(|val| val.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    vec![x0y0, x1y1]
}

pub fn solution_1(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| {
            let coords = get_coords(line);

            let x0y0 = &coords[0];
            let x1y1 = &coords[1];

            if x0y0[0] >= x1y1[0] && x0y0[1] <= x1y1[1] {
                return 1;
            }

            if x1y1[0] >= x0y0[0] && x1y1[1] <= x0y0[1] {
                return 1;
            }

            0
        })
        .sum::<usize>()
}

pub fn solution_2(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| {
            let coords = get_coords(line);

            let x0y0 = &coords[0];
            let x1y1 = &coords[1];

            if max(x0y0[0], x1y1[0]) <= min(x0y0[1], x1y1[1]) {
                return 1;
            }

            0
        })
        .sum::<usize>()
}
