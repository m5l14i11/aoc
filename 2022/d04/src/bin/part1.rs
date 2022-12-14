use d04::solution_1;

fn main() {
    let data = include_str!("../../input.txt").trim();
    let res = solution_1(&data);
    println!("{:?}", res)
}