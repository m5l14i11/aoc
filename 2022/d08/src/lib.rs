fn parse_trees(input: &str) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    let mut buffer = Vec::new();

    let mut chars = input.chars();
    
    while let Some(c) = chars.next() {
        if c == '\n' {
            if !buffer.is_empty() {
                result.push(buffer);
                buffer = Vec::new();
            }
        } else if let Some(number) = c.to_digit(10) {
            buffer.push(number);
        }
    }

    if !buffer.is_empty() {
        result.push(buffer);
    }

    result
}

pub fn solution_1(input: &str) -> usize {
    let trees = parse_trees(input);
    let max_length = trees.len() - 1;

    let mut visible_trees: Vec<Vec<bool>> = trees
        .iter()
        .enumerate()
        .map(|(i, tree_line)| {
            let line_max_length = tree_line.len() - 1;
            tree_line
                .iter()
                .enumerate()
                .map(|(line_i, _)| {
                    i == 0 || i == max_length || line_i == 0 || line_i == line_max_length
                })
                .collect()
        })
        .collect();

    for y in 0..trees.len() {
        let mut current_tree_size = 0;
        for x in 0..trees[0].len() {
            if x == 0 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }
    for y in (0..trees.len()).rev() {
        let mut current_tree_size = 0;
        for x in (0..trees[0].len()).rev() {
            if x == trees[0].len() - 1 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }

    for x in 0..trees.len() {
        let mut current_tree_size = 0;
        for y in 0..trees[0].len() {
            if y == 0 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }
    for x in (0..trees.len()).rev() {
        let mut current_tree_size = 0;
        for y in (0..trees[0].len()).rev() {
            if y == trees[0].len() - 1 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }

    visible_trees.iter().flatten().filter(|&&v| v).count()
}

pub fn solution_2(input: &str) -> usize {
    let trees = parse_trees(input);

    let mut high_score = 0;

    for (y_index, tree_line) in trees.iter().enumerate() {
        for (x_index, &treehouse_height) in tree_line.iter().enumerate() {
            let mut scores = [0, 0, 0, 0];

            for &tree in tree_line[..x_index].iter().rev() {
                if tree < treehouse_height {
                    scores[0] += 1;
                } else {
                    scores[0] += 1;
                    break;
                }
            }

            for &tree in tree_line[x_index + 1..].iter() {
                if tree < treehouse_height {
                    scores[1] += 1;
                } else {
                    scores[1] += 1;
                    break;
                }
            }

            for tree_line in trees[..y_index].iter().rev() {
                if tree_line[x_index] < treehouse_height {
                    scores[2] += 1;
                } else {
                    scores[2] += 1;
                    break;
                }
            }

            for tree_line in trees[y_index + 1..].iter() {
                if tree_line[x_index] < treehouse_height {
                    scores[3] += 1;
                } else {
                    scores[3] += 1;
                    break;
                }
            }

            let scenic_score: u32 = scores.iter().product();

            if scenic_score > high_score {
                high_score = scenic_score
            }
        }
    }

    high_score as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn solution_1_test() {
        assert_eq!(solution_1(TEST_DATA.trim()), 21);
    }

    #[test]
    fn solution_2_test() {
        assert_eq!(solution_2(TEST_DATA.trim()), 8);
    }
}