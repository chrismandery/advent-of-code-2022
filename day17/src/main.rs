use anyhow::{Context, Result};
use std::cmp::max;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

/// All coordinates are one-indexed, i.e. the lowest row is y=1 and x is in [1, 7]
type BlockPos = (i8, u64);
type Field = HashSet<BlockPos>;

const NUM_ROWS_STATE: u64 = 100;

#[derive(Debug, Eq, Hash, PartialEq)]
enum BlockType {
    HLine,
    Plus,
    Corner,
    VLine,
    Square
}

#[derive(Eq, Hash, PartialEq)]
struct BlockFallResult {
    block_type: BlockType,
    push_dir_index: usize,
    fallen_steps: u64,
    top_rows_state: BTreeSet<(i8, u64)>  // Contains the state of the top NUM_ROWS_STATE rows
}

/// Data structure used to detect cycles for the second part of the puzzle (the value stores the round and the height)
type CycleCheckingMap = HashMap<BlockFallResult, (u64, u64)>;

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

fn calc_height(f: &Field) -> u64 {
    f.iter().map(|(_, y)| *y).max().unwrap_or(0)
}

fn calc_height_after_rounds(push_directions: &[i8], num_rounds: u64) -> u64 {
    let mut f = Field::new();
    let mut push_dir_counter = 0;
    let mut known_states = CycleCheckingMap::new();
    let mut round = 0;
    let mut cycle_height_adder = None;

    while round < num_rounds {
        let cur_height = calc_height(&f);
        // println!("Round {}: cur_height={} + cycle_height_adder={:?}", round, cur_height, cycle_height_adder);

        let block_type = match round.rem_euclid(5) {
            0 => BlockType::HLine,
            1 => BlockType::Plus,
            2 => BlockType::Corner,
            3 => BlockType::VLine,
            4 => BlockType::Square,
            _ => panic!("mod 5 not in 0..=4?!")
        };

        let spawn_pos = (3, cur_height + 4);
        let fall_counter = simulate_block_fall(push_directions, &mut f, &mut push_dir_counter, &block_type, spawn_pos);
        // _print_field(&f);

        // Check for cycles
        let top_rows_base_y = if cur_height >= NUM_ROWS_STATE { cur_height - NUM_ROWS_STATE } else { 0 };
        let top_rows_state = f
            .iter()
            .filter_map(|(x, y)| {
                if *y >= top_rows_base_y {
                    Some((*x, y - top_rows_base_y))
                } else {
                    None
                }
            })
            .collect();

        let bfr = BlockFallResult {
            block_type: block_type,
            push_dir_index: push_dir_counter.rem_euclid(push_directions.len()),
            fallen_steps: fall_counter,
            top_rows_state
        };

        if cycle_height_adder.is_none() {
            if let Some((last_round, last_height)) = known_states.get(&bfr) {
                let cycle_blocks = round - last_round;
                let cycle_height_increase = cur_height - last_height;
                println!("Cycle found: Cycle of {} blocks leads to height increase of {}.", cycle_blocks, cycle_height_increase);
                println!("Cycle is defined by: block_type={:?} push_dir_index={} fallen_steps={} len(top_rows_state)={}",
                    bfr.block_type, bfr.push_dir_index, bfr.fallen_steps, bfr.top_rows_state.len());

                // Fast-forward by applying the cycle to skip computational effort
                let apply_cycles = max((num_rounds - round) / cycle_blocks, 1) - 1;
                cycle_height_adder = Some(apply_cycles * cycle_height_increase);
                round += apply_cycles * cycle_blocks;

                println!("Applying {} cycles to save computational effort: {} of additional height added.", apply_cycles,
                    cycle_height_adder.unwrap());
            } else {
                known_states.insert(bfr, (round, cur_height));
            }
        }

        round += 1;
    }

    calc_height(&f) + cycle_height_adder.unwrap_or(0)
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
    println!("Height of tower of rocks after 2022 blocks: {}", calc_height_after_rounds(&push_directions, 2022));
    println!("Height of tower of rocks after 1000000000000 blocks: {}", calc_height_after_rounds(&push_directions, 1000000000000));

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

/// Returns the number of fields the block has fallen (used for detecting cycles for the second half of the puzzle).
fn simulate_block_fall(push_directions: &[i8], f: &mut Field, push_dir_counter: &mut usize, block_type: &BlockType,
    spawn_pos: BlockPos) -> u64 {
    let mut cur_pos = spawn_pos;
    let mut fall_counter = 0;

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
        fall_counter += 1;
    }

    // Block has collided, at positions to field
    for offset in get_block_fields(&block_type) {
        if !f.insert((cur_pos.0 + offset.0, cur_pos.1 + offset.1)) {
            panic!("Attempted to set a position on the field that was already occupied! (should never happen)")
        }
    }

    fall_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let push_directions = read_input_file("../inputs/day17_example.txt").unwrap();
        assert_eq!(calc_height_after_rounds(&push_directions, 2022), 3068);
    }

    #[test]
    fn example_part2() {
        let push_directions = read_input_file("../inputs/day17_example.txt").unwrap();
        assert_eq!(calc_height_after_rounds(&push_directions, 1000000000000), 1514285714288);
    }
}
