use array2d::Array2D;
use std::fs::read_to_string;
use std::path::Path;

enum Dir {
    Up,
    Down,
    Left,
    Right
}

fn check_if_visible(grid: &Array2D<u8>, i: usize, j: usize, dir: Dir) -> bool {
    let other_trees_max = match dir {
        Dir::Up => (0..i).map(|x| grid.get(x, j).unwrap()).max(),
        Dir::Down => ((i+1)..grid.num_rows()).map(|x| grid.get(x, j).unwrap()).max(),
        Dir::Left => (0..j).map(|x| grid.get(i, x).unwrap()).max(),
        Dir::Right => ((j+1)..grid.num_columns()).map(|x| grid.get(i, x).unwrap()).max()
    };

    grid.get(i, j) > other_trees_max
}

fn get_max_scenic_score(grid: &Array2D<u8>) -> usize {
    let mut max_scenic_score = 0;

    // Do not consider trees on the border since there scenic score is zero anyway
    for i in 1..grid.num_rows()-1 {
        for j in 1..grid.num_columns()-1 {
            let scenic_score = get_scenic_score_for_dir(grid, i, j, Dir::Up) *
                get_scenic_score_for_dir(grid, i, j, Dir::Down) *
                get_scenic_score_for_dir(grid, i, j, Dir::Left) *
                get_scenic_score_for_dir(grid, i, j, Dir::Right);

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score
}

fn get_scenic_score_for_dir(grid: &Array2D<u8>, i: usize, j: usize, dir: Dir) -> usize {
    let own_height = grid.get(i, j).unwrap();

    let tree_heights: Vec<u8> = match dir {
        Dir::Up => (0..i).rev().map(|x| grid.get(x, j).unwrap().clone()).collect(),
        Dir::Down => ((i+1)..grid.num_rows()).map(|x| grid.get(x, j).unwrap().clone()).collect(),
        Dir::Left => (0..j).rev().map(|x| grid.get(i, x).unwrap().clone()).collect(),
        Dir::Right => ((j+1)..grid.num_columns()).map(|x| grid.get(i, x).unwrap().clone()).collect()
    };

    let smaller_trees = tree_heights.iter().take_while(|h| *h < own_height).count();

    if smaller_trees < tree_heights.len() {
        smaller_trees + 1
    } else {
        smaller_trees  // All trees until the border are smaller, so there is no higher tree to +1 for
    }
}

fn get_visible_tree_count(grid: &Array2D<u8>) -> usize {
    let mut count = 0;

    for i in 0..grid.num_rows() {
        for j in 0..grid.num_columns() {
            if check_if_visible(grid, i, j, Dir::Up) ||
                check_if_visible(grid, i, j, Dir::Down) ||
                check_if_visible(grid, i, j, Dir::Left) ||
                check_if_visible(grid, i, j, Dir::Right) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let grid = read_input_file("../inputs/day8_input.txt");
    println!("Number of visible trees: {}", get_visible_tree_count(&grid));
    println!("Highest scenic score: {}", get_max_scenic_score(&grid));
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Array2D<u8> {
    let input = read_to_string(input_path).expect("Could not read input file!");
    let rows: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).expect("Could not parse digit!") as u8).collect())
        .collect();
    Array2D::from_rows(&rows).expect("Could not build grid!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let grid = read_input_file("../inputs/day8_example.txt");
        assert_eq!(get_visible_tree_count(&grid), 21);
        assert_eq!(get_max_scenic_score(&grid), 8);
    }
}
