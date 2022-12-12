use anyhow::{anyhow, Context, Result};
use array2d::Array2D;
use std::cmp::min;
use std::fs::read_to_string;
use std::path::Path;

type Position = (usize, usize);

struct Input {
    height_map: Array2D<u8>,
    start_pos: Position,
    end_pos: Position
}

#[derive(Clone)]
struct DijkstraNode {
    distance_to_start: u32,
    visited: bool
}

/// Computes the length of the optimal path with dynamic programming (Dijkstra's algorithm)
/// The implementation is horribly inefficient and could be optimized using a heap structure as a node queue.
/// However, for the size of the AoC grid it does not really matter.
fn get_optimal_step_count(input: &Input) -> u32 {
    let row_count = input.height_map.num_rows();
    let col_count = input.height_map.num_columns();

    // Initialize grid
    let mut grid = Array2D::filled_with(DijkstraNode {
        distance_to_start: u32::MAX,
        visited: false
    }, row_count, col_count);
    grid.set(input.start_pos.0, input.start_pos.1, DijkstraNode { distance_to_start: 0, visited: false }).unwrap();

    loop {
        // Determine unvisited node with the lowest distance from the start position (inefficient to do it like this)
        let mut min_distance_value = None;
        let mut min_distance_pos = None;
        for i in 0..row_count {
            for j in 0..col_count {
                let n = grid.get(i, j).unwrap();
                if !n.visited && min_distance_value.map(|v| n.distance_to_start < v).unwrap_or(true) {
                    min_distance_value = Some(n.distance_to_start);
                    min_distance_pos = Some((i, j));
                }
            }
        }

        // No unvisited node found anymore and node to visit has max distance (cannot be reached)? -> Terminate
        if min_distance_value.is_none() || min_distance_value.unwrap() == u32::MAX {
            break;
        }


        // Mark node as visited and store height
        let min_distance_pos = min_distance_pos.unwrap();
        grid.get_mut(min_distance_pos.0, min_distance_pos.1).unwrap().visited = true;
        let cur_height = input.height_map.get(min_distance_pos.0, min_distance_pos.1).unwrap();
        // println!("Current node is {:?} with a height of {} and a distance of {}.", &min_distance_pos, cur_height, min_distance_value.unwrap());

        // Check neighbors of current node
        for (offset_row, offset_col) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let neighbor_row = min_distance_pos.0 as isize + offset_row;
            let neighbor_col = min_distance_pos.1 as isize + offset_col;

            if neighbor_row >= 0 && neighbor_row < row_count as isize && neighbor_col >= 0 && neighbor_col < col_count as isize {
                // Check if neighbor is reachable (height <= current height + 1)
                let neighbor_height = *input.height_map.get(neighbor_row as usize, neighbor_col as usize).unwrap();
                // println!("Checking neighbor at {}/{}: Its height is {}.", neighbor_row, neighbor_col, neighbor_height);
                if neighbor_height > cur_height + 1 {
                    continue;
                }

                // Update distance if we found a better way
                let neighbor = grid.get_mut(neighbor_row as usize, neighbor_col as usize).unwrap();
                neighbor.distance_to_start = min(neighbor.distance_to_start, min_distance_value.unwrap() + 1);
                // println!("Updating neighbor: Its distance is now {}.", neighbor.distance_to_start);
            }
        }
    }

    // Determine distance between start and end position
    grid.get(input.end_pos.0, input.end_pos.1).unwrap().distance_to_start
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day12_input.txt")?;
    println!("Number of steps required: {}", get_optimal_step_count(&input));

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Input> {
    let mut start_pos = None;
    let mut end_pos = None;

    let input = read_to_string(input_path).context("Could not read input file!")?;
    let rows: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(row_num, l)| l
            .chars()
            .enumerate()
            .map(|(col_num, c)| {
                if c == 'S' {
                    start_pos = Some((row_num, col_num));
                    0
                }
                else if c == 'E' {
                    end_pos = Some((row_num, col_num));
                    25
                }
                else {
                    c as u8 - 'a' as u8  // Convert to digit where 'a' is 0 and 'z' is 25
                }
            }).collect())
        .collect();

    match Array2D::from_rows(&rows) {
        Ok(hm) => {
            Ok(Input {
                height_map: hm,
                start_pos: start_pos.ok_or_else(|| anyhow!("No start position found!"))?,
                end_pos: end_pos.ok_or_else(|| anyhow!("No end position found!"))?
            })
        },
        Err(_) => {
            Err(anyhow!("Could not build height map!"))  // array2d's error type is not compatible with anyhow (does not implement std::error::Error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = read_input_file("../inputs/day12_example.txt").unwrap();
        assert_eq!(get_optimal_step_count(&input), 31);
    }
}
