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
    }
}
