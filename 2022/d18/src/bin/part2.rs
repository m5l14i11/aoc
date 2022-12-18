use d18::solution_2;

fn main() {
    let data = include_str!("../../input.txt").trim();
    let res = solution_2(&data);
    println!("{:?}", res)
}