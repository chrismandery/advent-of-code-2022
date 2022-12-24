use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

type Field = HashSet<(i32, i32)>;

#[derive(Debug)]
enum Direction {
    North, South, West, East
}

fn _debug_print_field(f: &Field) {
    let x_min = *f.iter().map(|(x, _)| x).min().unwrap() - 1;
    let x_max = *f.iter().map(|(x, _)| x).max().unwrap() + 1;
    let y_min = *f.iter().map(|(_, y)| y).min().unwrap() - 1;
    let y_max = *f.iter().map(|(_, y)| y).max().unwrap() + 1;

    println!("\nCurrent state of board:");
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if f.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn count_empty_ground_tiles(f: &Field) -> usize {
    let x_min = *f.iter().map(|(x, _)| x).min().unwrap();
    let x_max = *f.iter().map(|(x, _)| x).max().unwrap();
    let y_min = *f.iter().map(|(_, y)| y).min().unwrap();
    let y_max = *f.iter().map(|(_, y)| y).max().unwrap();

    let mut empty_count = 0;
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if !f.contains(&(x, y)) {
                empty_count += 1;
            }
        }
    }

    empty_count
}

fn count_rounds_before_stable(field: &Field) -> usize {
    let mut cur_field = field.clone();
    let mut dir_check_order = vec!(Direction::North, Direction::South, Direction::West, Direction::East);
    let mut n = 1;

    loop {
        let new_field = simulate_round(&cur_field, &dir_check_order);
        if new_field == cur_field {
            return n;
        }

        cur_field = new_field;

        // Rotate list of directions
        let dir = dir_check_order.remove(0);
        dir_check_order.push(dir);

        n += 1;
    }
}

fn get_elf_movement_offset(f: &Field, pos: (i32, i32), dir_check_order: &[Direction]) -> (i32, i32) {
    // Check if elf does not have any adjacent elf and should not move
    if !f.contains(&(pos.0 - 1, pos.1 - 1)) && !f.contains(&(pos.0, pos.1 - 1)) && !f.contains(&(pos.0 + 1, pos.1 - 1)) &&
        !f.contains(&(pos.0 - 1, pos.1)) && !f.contains(&(pos.0 + 1, pos.1)) &&
        !f.contains(&(pos.0 - 1, pos.1 + 1)) && !f.contains(&(pos.0, pos.1 + 1)) && !f.contains(&(pos.0 + 1, pos.1 + 1)) {
        return (0, 0)
    }

    for dir in dir_check_order {
        match dir {
            Direction::North => {
                if !f.contains(&(pos.0 - 1, pos.1 - 1)) && !f.contains(&(pos.0, pos.1 - 1)) && !f.contains(&(pos.0 + 1, pos.1 - 1)) {
                    return (0, -1);
                }
            },
            Direction::South => {
                if !f.contains(&(pos.0 - 1, pos.1 + 1)) && !f.contains(&(pos.0, pos.1 + 1)) && !f.contains(&(pos.0 + 1, pos.1 + 1)) {
                    return (0, 1);
                }
            },
            Direction::West => {
                if !f.contains(&(pos.0 - 1, pos.1 - 1)) && !f.contains(&(pos.0 - 1, pos.1)) && !f.contains(&(pos.0 - 1, pos.1 + 1)) {
                    return (-1, 0);
                }
            },
            Direction::East => {
                if !f.contains(&(pos.0 + 1, pos.1 - 1)) && !f.contains(&(pos.0 + 1, pos.1)) && !f.contains(&(pos.0 + 1, pos.1 + 1)) {
                    return (1, 0);
                }
            }
        }
    }

    // No move possible
    (0, 0)
}

fn main() -> Result<()> {
    let elf_pos = read_input_file("../inputs/day23_input.txt").unwrap();
    let elf_pos = simulate_n_rounds(&elf_pos, 10);
    println!("Empty ground tiles after ten rounds: {}", count_empty_ground_tiles(&elf_pos));

    let elf_pos = read_input_file("../inputs/day23_input.txt").unwrap();
    println!("First round where no Elf moves anymore: {}", count_rounds_before_stable(&elf_pos));

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Field> {
    let input = read_to_string(input_path).expect("Could not read input file!");

    let field = HashSet::from_iter(input
        .lines()
        .enumerate()
        .flat_map(move |(row_num, l)| l
            .chars()
            .enumerate()
            .filter_map(move |(col_num, c)| {
                if c == '#' {
                    Some((col_num as i32, row_num as i32))
                }
                else if c == '.' {
                    None
                } else {
                    panic!("Unknown input character!")
                }
            })));

    Ok(field)
}

fn simulate_n_rounds(field: &Field, n: usize) -> Field {
    // _debug_print_field(&field);

    let mut cur_field = field.clone();
    let mut dir_check_order = vec!(Direction::North, Direction::South, Direction::West, Direction::East);

    for _ in 0..n {
        cur_field = simulate_round(&cur_field, &dir_check_order);

        // Rotate list of directions
        let dir = dir_check_order.remove(0);
        dir_check_order.push(dir);

        // _debug_print_field(&cur_field);
    }

    cur_field
}

fn simulate_round(field: &Field, dir_check_order: &[Direction]) -> Field {
    // Count how many elves want to move to a specific field
    let mut target_count = HashMap::new();
    for pos in field {
        let offset = get_elf_movement_offset(field, *pos, dir_check_order);
        let new_pos = (pos.0 + offset.0, pos.1 + offset.1);

        match target_count.get(&new_pos) {
            Some(c) => { target_count.insert(new_pos, c + 1); }
            None => { target_count.insert(new_pos, 1); }
        }
    }

    // Build new field where each elf is moved if the target count is only one
    let mut new_field = HashSet::new();
    for pos in field {
        let offset = get_elf_movement_offset(field, *pos, dir_check_order);
        let new_pos = (pos.0 + offset.0, pos.1 + offset.1);

        if *target_count.get(&new_pos).unwrap() == 1 {
            new_field.insert(new_pos);
        } else {
            new_field.insert(*pos);
        }
    }

    new_field
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let elf_pos = read_input_file("../inputs/day23_example.txt").unwrap();
        let elf_pos = simulate_n_rounds(&elf_pos, 10);
        assert_eq!(count_empty_ground_tiles(&elf_pos), 110);
    }

    #[test]
    fn example_part1_small() {
        let mut correct_end_state = HashSet::new();
        correct_end_state.insert((2, 5));
        correct_end_state.insert((0, 2));
        correct_end_state.insert((4, 1));
        correct_end_state.insert((2, 0));
        correct_end_state.insert((4, 3));

        let elf_pos = read_input_file("../inputs/day23_example_small.txt").unwrap();
        let elf_pos = simulate_n_rounds(&elf_pos, 3);
        assert_eq!(elf_pos, correct_end_state);
    }

    #[test]
    fn example_part2() {
        let elf_pos = read_input_file("../inputs/day23_example.txt").unwrap();
        assert_eq!(count_rounds_before_stable(&elf_pos), 20);
    }
}
