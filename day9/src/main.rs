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

fn calc_visited_tail_fields(moves: &[Direction]) -> HashSet<(i32, i32)> {
    let mut pos_head: (i32, i32) = (0, 0);
    let mut pos_tail: (i32, i32) = (0, 0);
    let mut visited_tail_fields = HashSet::new();
    visited_tail_fields.insert(pos_tail);

    for dir in moves {
        // Move head
        match dir {
            Direction::Up => { pos_head.1 -= 1; }
            Direction::Down => { pos_head.1 += 1; }
            Direction::Left => { pos_head.0 -= 1; }
            Direction::Right => { pos_head.0 += 1; }
        }

        // Check if tail is too far away and needs to be moved
        if (pos_head.0 - pos_tail.0).abs() > 1 || (pos_head.1 - pos_tail.1).abs() > 1 {
            // Set tail to field beside head based on movement direction
            match dir {
                Direction::Up => { pos_tail = (pos_head.0, pos_head.1 + 1) }
                Direction::Down => { pos_tail = (pos_head.0, pos_head.1 - 1) }
                Direction::Left => { pos_tail = (pos_head.0 + 1, pos_head.1) }
                Direction::Right => { pos_tail = (pos_head.0 - 1, pos_head.1) }
            }
        }

        visited_tail_fields.insert(pos_tail);
    }

    visited_tail_fields
}

fn main() -> Result<()> {
    let moves = read_input_file("../inputs/day9_input.txt")?;
    let visited_tail_fields = calc_visited_tail_fields(&moves);
    println!("Number of fields visited by the rope tail: {}", visited_tail_fields.len());

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
    fn example() {
        let moves = read_input_file("../inputs/day9_example.txt").unwrap();
        let visited_tail_fields = calc_visited_tail_fields(&moves);
        assert_eq!(visited_tail_fields.len(), 13);
    }
}
