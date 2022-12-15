use std::cmp::{max, min};

fn get_coords(line: &str) -> Vec<Vec<usize>> {
    line.split(',')
        .map(|d| {
            d.split('-')
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
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
