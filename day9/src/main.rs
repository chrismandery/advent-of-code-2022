use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn calc_visited_tail_fields(moves: &[Direction], tail_count: usize) -> HashSet<(i32, i32)> {
    // First index is the head, all the other ones are the tails
    let mut positions: Vec<(i32, i32)> = std::vec::from_elem((0, 0), tail_count + 1);
    let mut visited_tail_fields = HashSet::new();

    for dir in moves {
        // Move head according to given movement direction
        match dir {
            Direction::Up => { positions[0].1 -= 1; }
            Direction::Down => { positions[0].1 += 1; }
            Direction::Left => { positions[0].0 -= 1; }
            Direction::Right => { positions[0].0 += 1; }
        }

        // Move all tails from head to tail
        for i in 1..=tail_count {
            // Calculate delta to earlier tail (or head)
            let delta = (positions[i - 1].0 - positions[i].0, positions[i - 1].1 - positions[i].1);

            // Move tail according to movement of earlier tail (or head) if tail is too far away and needs to be moved
            if delta.0.abs() > 1 || delta.1.abs() > 1 {
                // Perform movement of tail segment where both axes are clamped at -1/1
                let clamped_delta = (delta.0.signum(), delta.1.signum());
                positions[i] = (positions[i].0 + clamped_delta.0, positions[i].1 + clamped_delta.1)
            }
        }

        // Output all positions for debugging
        /* let pos_str: Vec<String> = positions.iter().map(|(x, y)| format!("({}, {})", x, y)).collect();
        println!("{}", pos_str.join(" - ")); */

        // Record position of final tail
        visited_tail_fields.insert(positions.iter().last().unwrap().clone());
    }

    visited_tail_fields
}

fn main() -> Result<()> {
    let moves = read_input_file("../inputs/day9_input.txt")?;

    let visited_tail_fields = calc_visited_tail_fields(&moves, 1);
    println!("Number of fields visited by the rope tail (length 1): {}", visited_tail_fields.len());

    let visited_tail_fields = calc_visited_tail_fields(&moves, 9);
    println!("Number of fields visited by the rope tail (length 9): {}", visited_tail_fields.len());

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<Direction>> {
    let re = Regex::new(r"^(U|D|L|R) (\d+)$")?;
    let input = read_to_string(input_path).context("Could not read input file!")?;
    let mut res = vec!();

    for l in input.lines() {
        let c = re.captures(l).ok_or_else(|| anyhow!("Could not parse line: {}", l))?;

        let dir = match c.get(1).unwrap().as_str() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => bail!("Unknown direction!")
        };
        let count: usize = c.get(2).unwrap().as_str().parse()?;

        for _ in 0..count {
            res.push(dir.clone());
        };
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_small() {
        let moves = read_input_file("../inputs/day9_example1.txt").unwrap();

        let visited_tail_fields = calc_visited_tail_fields(&moves, 1);
        assert_eq!(visited_tail_fields.len(), 13);

        let visited_tail_fields = calc_visited_tail_fields(&moves, 9);
        assert_eq!(visited_tail_fields.len(), 1);
    }

    #[test]
    fn example_large() {
        let moves = read_input_file("../inputs/day9_example2.txt").unwrap();

        let visited_tail_fields = calc_visited_tail_fields(&moves, 9);
        assert_eq!(visited_tail_fields.len(), 36);
    }
}
