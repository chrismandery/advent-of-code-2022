use anyhow::{bail, Result};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

enum Direction {
    North, South, West, East
}

type Pos = (i32, i32);

struct Blizzard {
    pos: Pos,
    dir: Direction
}

fn _debug_print_field(blizzards_at_pos: &HashSet<Pos>, field_size: &Pos, start_pos: &Pos, end_pos: &Pos, cur_pos: &HashSet<Pos>) {
    println!("\nCurrent field state:");

    for y in 0..field_size.1 {
        for x in 0..field_size.0 {
            let c =
                if cur_pos.contains(&(x, y)) { 'X' }
                else if blizzards_at_pos.contains(&(x, y)) { 'b' }
                else {
                    if y == 0 {
                        if x == start_pos.0 { ' ' } else { '#' }
                    } else if y == field_size.1 - 1 {
                        if x == end_pos.0 { ' ' } else { '#' }
                    } else {
                        if x == 0 || x == field_size.0 - 1 { '#' }
                        else { ' ' }
                    }
                };

            print!("{}", c);
        }

        println!("");
    }
}

/// Returns the number of steps necessary to reach the end position.
fn find_way(blizzards_at_pos: &[HashSet<Pos>], field_size: &Pos, start_pos: &Pos, end_pos: &Pos, start_step_count: usize) -> usize {
    let mut step_count = start_step_count;

    // Track positions reached in the current step for a BFS
    let mut cur_pos = HashSet::new();
    cur_pos.insert(*start_pos);

    while !cur_pos.contains(end_pos) {
        // _debug_print_field(&blizzards_at_pos[step_count % blizzards_at_pos.len()], field_size, start_pos, end_pos, &cur_pos);
        step_count += 1;

        // Determine all positions reachable from current positions in the next step
        let mut next_pos = HashSet::new();

        for pos in cur_pos {
            for offset in [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
                if ((new_pos == *start_pos || new_pos == *end_pos) ||
                    (new_pos.0 >= 1 && new_pos.0 <= field_size.0 - 2 && new_pos.1 >= 1 && new_pos.1 <= field_size.1 - 2)) &&
                    blizzards_at_pos[step_count % blizzards_at_pos.len()].get(&new_pos).is_none() {
                    next_pos.insert(new_pos);
                }
            }
        }

        cur_pos = next_pos;
    }

    step_count
}

fn main() -> Result<()> {
    let (mut blizzards, field_size, start_pos, end_pos) = read_input_file("../inputs/day24_input.txt").unwrap();
    let blizzards_at_round = precalc_blizzard_pos(&mut blizzards, &field_size);

    let part1_steps = find_way(&blizzards_at_round, &field_size, &start_pos, &end_pos, 0);
    println!("Part 1 - Number of steps to end position: {}", part1_steps);

    let part2_steps_reach_start = find_way(&blizzards_at_round, &field_size, &end_pos, &start_pos, part1_steps);
    let part2_steps_reach_end = find_way(&blizzards_at_round, &field_size, &start_pos, &end_pos, part2_steps_reach_start);
    println!("Part 2 - Number of steps to end, start and end again: {}", part2_steps_reach_end);

    Ok(())
}

/// Calculates the positions of blizzards for a total of X*Y rounds where X and Y is the row/colum size of the field (could also use the
/// least common multiple here).
fn precalc_blizzard_pos(blizzards: &mut Vec<Blizzard>, field_size: &Pos) -> Vec<HashSet<Pos>> {
    let mut res = vec!();

    for _ in 0..(field_size.0 * field_size.1) {
        let mut blizzard_pos = HashSet::new();

        for b in blizzards.iter_mut() {
            blizzard_pos.insert(b.pos);

            // Move blizzard
            match b.dir {
                Direction::North => {
                    b.pos.1 = if b.pos.1 == 1 { field_size.1 - 2 } else { b.pos.1 - 1 };
                },
                Direction::South => {
                    b.pos.1 = if b.pos.1 == field_size.1 - 2 { 1 } else { b.pos.1 + 1 };
                }
                Direction::West => {
                    b.pos.0 = if b.pos.0 == 1 { field_size.0 - 2 } else { b.pos.0 - 1 };
                }
                Direction::East => {
                    b.pos.0 = if b.pos.0 == field_size.0 - 2 { 1 } else { b.pos.0 + 1 };
                }
            }
        }

        res.push(blizzard_pos);
    }

    res
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<(Vec<Blizzard>, Pos, Pos, Pos)> {
    let input = read_to_string(input_path).expect("Could not read input file!");
    let mut lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();

    // Remove top and bottom row and determine start and end position
    let top_line = lines.remove(0);
    let bottom_line = lines.pop().unwrap();
    if top_line.len() != bottom_line.len() {
        bail!("Line length not consistent!");
    }

    let start_pos = (top_line.chars().position(|c| c == '.').expect("Start position not found!") as i32, 0);
    let end_pos = (bottom_line.chars().position(|c| c == '.').expect("End position not found!") as i32, lines.len() as i32 + 1);
    let field_size = (top_line.len() as i32, lines.len() as i32 + 2);

    // Parse information about blizzards
    let mut blizzards = vec!();

    for (y, line) in lines.iter().enumerate() {
        if line.len() as i32 != field_size.0 {
            bail!("Line length not consistent!");
        }

        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => { blizzards.push(Blizzard { pos: (x as i32, y as i32 + 1), dir: Direction::North }) },
                'v' => { blizzards.push(Blizzard { pos: (x as i32, y as i32 + 1), dir: Direction::South }) },
                '<' => { blizzards.push(Blizzard { pos: (x as i32, y as i32 + 1), dir: Direction::West }) },
                '>' => { blizzards.push(Blizzard { pos: (x as i32, y as i32 + 1), dir: Direction::East }) },
                _ => {}  // Ignore other characters
            }
        }
    }

    Ok((blizzards, field_size, start_pos, end_pos))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (mut blizzards, field_size, start_pos, end_pos) = read_input_file("../inputs/day24_example.txt").unwrap();
        let blizzards_at_round = precalc_blizzard_pos(&mut blizzards, &field_size);

        // Part 1
        assert_eq!(find_way( &blizzards_at_round, &field_size, &start_pos, &end_pos, 0), 18);

        // Part 2
        assert_eq!(find_way( &blizzards_at_round, &field_size, &end_pos, &start_pos, 18), 18 + 23);
        assert_eq!(find_way( &blizzards_at_round, &field_size, &start_pos, &end_pos, 41), 41 + 13);
    }
}
