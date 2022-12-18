use anyhow::{Context, Result};
use std::collections::{HashSet};
use std::fs::read_to_string;
use std::path::Path;

/// All coordinates are one-indexed, i.e. the lowest row is y=1 and x is in [1, 7]
type BlockPos = (i8, u32);
type Field = HashSet<BlockPos>;

enum BlockType {
    HLine,
    Plus,
    Corner,
    VLine,
    Square
}

/// Only used for debugging
fn _print_field(f: &Field) {
    println!("\nCurrent field is:");
    let mut y = calc_height(f);

    while y > 0 {
        print!("|");
        for x in 1..=7 {
            if f.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("|");
        y -= 1;
    }

    println!("+-------+");
}

fn calc_height(f: &Field) -> u32 {
    f.iter().map(|(_, y)| *y).max().unwrap_or(0)
}

fn calc_height_after_rounds(push_directions: &[i8], num_rounds: u32) -> u32 {
    let mut f = Field::new();
    let mut push_dir_counter = 0;

    for round in 0..num_rounds {
        let block_type = match round.rem_euclid(5) {
            0 => BlockType::HLine,
            1 => BlockType::Plus,
            2 => BlockType::Corner,
            3 => BlockType::VLine,
            4 => BlockType::Square,
            _ => panic!("mod 5 not in 0..=4?!")
        };

        let spawn_pos = (3, calc_height(&f) + 4);
        simulate_block_fall(push_directions, &mut f, &mut push_dir_counter, &block_type, spawn_pos);
        // _print_field(&f);
    }

    calc_height(&f)
}

fn check_block_collision(f: &mut Field, block_type: &BlockType, block_pos: BlockPos) -> bool {
    for offset in get_block_fields(block_type) {
        if f.contains(&(block_pos.0 + offset.0, block_pos.1 + offset.1)) {
            return true;
        }
    }

    false
}

fn get_block_fields(block_type: &BlockType) -> Vec<BlockPos> {
    match block_type {
        BlockType::HLine => vec!((0, 0), (1, 0), (2, 0), (3, 0)),
        BlockType::Plus => vec!((0, 1), (1, 0), (1, 1), (1, 2), (2, 1)),
        BlockType::Corner => vec!((0, 0), (1, 0), (2, 0), (2, 1), (2, 2)),
        BlockType::VLine => vec!((0, 0), (0, 1), (0, 2), (0, 3)),
        BlockType::Square => vec!((0, 0), (1, 0), (0, 1), (1, 1))
    }
}

fn get_block_width(block_type: &BlockType) -> i8 {
    match block_type {
        BlockType::HLine => 4,
        BlockType::Plus => 3,
        BlockType::Corner => 3,
        BlockType::VLine => 1,
        BlockType::Square => 2
    }
}

fn main() -> Result<()> {
    let push_directions = read_input_file("../inputs/day17_input.txt")?;
    println!("Height of tower of rocks: {}", calc_height_after_rounds(&push_directions, 2022));

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<i8>> {
    let input = read_to_string(input_path).context("Could not read input file!")?;
    let res = input.chars().into_iter().filter_map(|c|
        match c {
            '<' => Some(-1),
            '>' => Some(1),
            _ => None
        }
    ).collect();

    Ok(res)
}

fn simulate_block_fall(push_directions: &[i8], f: &mut Field, push_dir_counter: &mut usize, block_type: &BlockType,
    spawn_pos: BlockPos) {
    let mut cur_pos = spawn_pos;

    loop {
        // Push horizontally
        let push_dir = push_directions.get(push_dir_counter.rem_euclid(push_directions.len())).unwrap();
        let push_pos = (cur_pos.0 + push_dir, cur_pos.1);
        *push_dir_counter += 1;

        if push_pos.0 >= 1 && push_pos.0 + get_block_width(block_type) <= 8 && !check_block_collision(f, &block_type, push_pos) {
            cur_pos = push_pos;
        }

        // Fall down vertically
        let falldown_pos = (cur_pos.0, cur_pos.1 - 1);
        if falldown_pos.1 == 0 || check_block_collision(f, &block_type, falldown_pos) {
            break;
        }
        cur_pos = falldown_pos;
    }

    // Block has collided, at positions to field
    for offset in get_block_fields(&block_type) {
        if !f.insert((cur_pos.0 + offset.0, cur_pos.1 + offset.1)) {
            panic!("Attempted to set a position on the field that was already occupied! (should never happen)")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let push_directions = read_input_file("../inputs/day17_example.txt").unwrap();
        assert_eq!(calc_height_after_rounds(&push_directions, 2022), 3068);
    }
}
