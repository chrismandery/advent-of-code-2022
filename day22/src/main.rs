use anyhow::{bail, Result};
use array2d::Array2D;
use std::fs::read_to_string;
use std::ops::Range;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
enum Field {
    Empty,
    Blocked,
    OffMap
}

type Board = Array2D<Field>;

enum Move {
    Forward { steps: usize },
    Turn { turn_right: bool }
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Debug)]
struct Position {
    row: usize,
    column: usize,
    dir: Direction
}

fn get_final_position(board: &Board, moves: &[Move], cube_overflow: bool) -> Position {
    // Determine start position
    let mut cur_pos = Position {
        row: 0,
        column: board.row_iter(0).unwrap().position(|x| *x == Field::Empty).expect("No free field found in top row!"),
        dir: Direction::Right
    };

    // Apply all moves
    for m in moves {
        match m {
            Move::Forward { steps } => {
                for _ in 0..*steps {
                    if let Some(new_pos) = move_forward_one_step(&board, &cur_pos, cube_overflow) {
                        cur_pos = new_pos;
                        // println!("Moved to: x={} y={} {:?}", cur_pos.column, cur_pos.row, cur_pos.dir);
                    } else {
                        break;
                    }
                }
            },
            Move::Turn { turn_right } => {
                cur_pos.dir = match cur_pos.dir {
                    Direction::Up => { if *turn_right { Direction::Right } else { Direction::Left } },
                    Direction::Right => { if *turn_right { Direction::Down } else { Direction::Up } },
                    Direction::Down => { if *turn_right { Direction::Left } else { Direction::Right } },
                    Direction::Left => { if *turn_right { Direction::Up } else { Direction::Down } }
                };
            }
        }
    }

    cur_pos
}

fn get_first_valid_in_range_x(board: &Board, x: Range<usize>, y: usize) -> usize {
    x.clone().find(|i| *board.get(y, *i).unwrap() != Field::OffMap).expect("No overflow field found?!")
}

fn get_first_valid_in_range_y(board: &Board, x: usize, y: Range<usize>) -> usize {
    y.clone().find(|i| *board.get(*i, x).unwrap() != Field::OffMap).expect("No overflow field found?!")
}

fn get_last_valid_in_range_x(board: &Board, x: Range<usize>, y: usize) -> usize {
    x.rev().find(|i| *board.get(y, *i).unwrap() != Field::OffMap).expect("No overflow field found?!")
}

fn get_last_valid_in_range_y(board: &Board, x: usize, y: Range<usize>) -> usize {
    y.rev().find(|i| *board.get(*i, x).unwrap() != Field::OffMap).expect("No overflow field found?!")
}

/// Determine next field to potentially move to, potentially overflowing (ugly branching structure, to be implemented more elegantly)
fn get_next_field(board: &Board, pos: &Position, cube_overflow: bool) -> Position {
    match pos.dir {
        Direction::Right => {
            // Overflow at the right border over the board or if we would be off map
            if pos.column != board.num_columns() - 1 && *board.get(pos.row, pos.column + 1).unwrap() != Field::OffMap {
                return Position { row: pos.row, column: pos.column + 1, dir: pos.dir.clone() };
            }
        },
        Direction::Down => {
            // Overflow at the bottom border over the board or if we would be off map
            if pos.row != board.num_rows() - 1 && *board.get(pos.row + 1, pos.column).unwrap() != Field::OffMap {
                return Position { row: pos.row + 1, column: pos.column, dir: pos.dir.clone() };
            }
        },
        Direction::Left => {
            // Overflow at the left border over the board or if we would be off map
            if pos.column != 0 && *board.get(pos.row, pos.column - 1).unwrap() != Field::OffMap {
                return Position { row: pos.row, column: pos.column - 1, dir: pos.dir.clone() };
            }
        },
        Direction::Up => {
            // Overflow at the top border over the board or if we would be off map
            if pos.row != 0 && *board.get(pos.row - 1, pos.column).unwrap() != Field::OffMap {
                return Position { row: pos.row - 1, column: pos.column, dir: pos.dir.clone() };
            }
        }
    }

    // Handle overflow
    if cube_overflow {
        get_overflow_field_cube(board, pos)
    } else {
        get_overflow_field(board, pos)
    }
}

/// Determines the "overflow field", i.e. the first field that is not off map (again, an ugly branching structure).
fn get_overflow_field(board: &Board, pos: &Position) -> Position {
    match pos.dir {
        Direction::Right => {
            Position {
                row: pos.row,
                column: board
                    .row_iter(pos.row)
                    .unwrap()
                    .position(|f| *f != Field::OffMap)
                    .expect("No overflow field found?!"),
                dir: pos.dir.clone()
            }
        },
        Direction::Down => {
            Position {
                row: board
                    .column_iter(pos.column)
                    .unwrap()
                    .position(|f| *f != Field::OffMap)
                    .expect("No overflow field found?!"),
                column: pos.column,
                dir: pos.dir.clone()
            }
        },
        Direction::Left => {
            Position {
                row: pos.row,
                column: board.num_columns() - 1 - board
                    .row_iter(pos.row)
                    .unwrap()
                    .rev()
                    .position(|f| *f != Field::OffMap)
                    .expect("No overflow field found?!"),
                dir: pos.dir.clone()
            }
        },
        Direction::Up => {
            Position {
                row: board.num_rows() - 1 - board
                    .column_iter(pos.column)
                    .unwrap()
                    .rev()
                    .position(|f| *f != Field::OffMap)
                    .expect("No overflow field found?!"),
                column: pos.column,
                dir: pos.dir.clone()
            }
        }
    }
}

/// Determines the "overflow field" for the second part of the puzzle.
fn get_overflow_field_cube(board: &Board, pos: &Position) -> Position {
    // Hardcoded cube layout
    //  12
    //  3
    // 45
    // 6

    // Calculate side length of the cube to determine how we are overflowing
    let cube_sl = board.num_rows() / 4;
    if board.num_columns() != cube_sl * 3 { panic!("Board does not fit hardcoded cube structure!") }

    let cube_face = if pos.row < cube_sl {
        if pos.column < cube_sl { panic!("Invalid position, check hardcoded cube faces!"); }
        else if pos.column < 2 * cube_sl { 1 }
        else { 2 }
    } else if pos.row < 2 * cube_sl {
        if pos.column < cube_sl { panic!("Invalid position, check hardcoded cube faces!"); }
        else if pos.column < 2 * cube_sl { 3 }
        else { panic!("Invalid position, check hardcoded cube faces!"); }
    } else if pos.row < 3 * cube_sl {
        if pos.column < cube_sl { 4 }
        else if pos.column < 2 * cube_sl { 5 }
        else { panic!("Invalid position, check hardcoded cube faces!"); }
    } else {
        if pos.column < cube_sl { 6 }
        else { panic!("Invalid position, check hardcoded cube faces!"); }
    };

    // Hardcode all possible transitions
    match (cube_face, &pos.dir) {
        (1, Direction::Up) => {
            Position {
                row: pos.column + 2 * cube_sl,
                column: get_first_valid_in_range_x(board, 0..(cube_sl - 1), pos.column + 2 * cube_sl),
                dir: Direction::Right
            }
        },
        (1, Direction::Left) => {
            Position {
                row: 3 * cube_sl - 1 - pos.row,
                column: get_first_valid_in_range_x(board, 0..(cube_sl - 1), 3 * cube_sl - 1 - pos.row),
                dir: Direction::Right
            }
        },
        (2, Direction::Up) => {
            Position {
                row: get_last_valid_in_range_y(board, pos.column - 2 * cube_sl, (3 * cube_sl)..(4 * cube_sl - 1)),
                column: pos.column - 2 * cube_sl,
                dir: Direction::Up
            }
        },
        (2, Direction::Right) => {
            Position {
                row: 3 * cube_sl - 1 - pos.row,
                column: get_last_valid_in_range_x(board, cube_sl..(2 * cube_sl - 1), 3 * cube_sl - 1 - pos.row),
                dir: Direction::Left
            }
        },
        (2, Direction::Down) => {
            Position {
                row: pos.column - cube_sl,
                column: get_last_valid_in_range_x(board, cube_sl..(2 * cube_sl - 1), pos.column - cube_sl),
                dir: Direction::Left
            }
        },
        (3, Direction::Right) => {
            Position {
                row: get_last_valid_in_range_y(board, pos.row + cube_sl, 0..(cube_sl - 1)),
                column: pos.row + cube_sl,
                dir: Direction::Up
            }
        },
        (3, Direction::Left) => {
            Position {
                row: get_first_valid_in_range_y(board, pos.row - cube_sl, (2 * cube_sl)..(3 * cube_sl - 1)),
                column: pos.row - cube_sl,
                dir: Direction::Down
            }
        },
        (4, Direction::Up) => {
            Position {
                row: pos.column + cube_sl,
                column: get_first_valid_in_range_x(board, cube_sl..(2 * cube_sl - 1), pos.column + cube_sl),
                dir: Direction::Right
            }
        },
        (4, Direction::Left) => {
            Position {
                row: 3 * cube_sl - 1 - pos.row,
                column: get_first_valid_in_range_x(board, cube_sl..(2 * cube_sl - 1), 3 * cube_sl - 1 - pos.row),
                dir: Direction::Right
            }
        },
        (5, Direction::Right) => {
            Position {
                row: 3 * cube_sl - 1 - pos.row,
                column: get_last_valid_in_range_x(board, (2 * cube_sl)..(3 * cube_sl - 1), 3 * cube_sl - 1 - pos.row),
                dir: Direction::Left
            }
        },
        (5, Direction::Down) => {
            Position {
                row: pos.row + cube_sl,
                column: get_last_valid_in_range_x(board, 0..(cube_sl - 1), pos.row + cube_sl),
                dir: Direction::Left
            }
        },
        (6, Direction::Right) => {
            Position {
                row: get_last_valid_in_range_y(board, pos.row - 2 * cube_sl, (2 * cube_sl)..(3 * cube_sl - 1)),
                column: pos.row - 2 * cube_sl,
                dir: Direction::Up
            }
        },
        (6, Direction::Down) => {
            Position {
                row: get_first_valid_in_range_y(board, pos.column + cube_sl, 0..(cube_sl - 1)),
                column: pos.column + cube_sl,
                dir: Direction::Down
            }
        },
        (6, Direction::Left) => {
            Position {
                row: get_first_valid_in_range_y(board, pos.row - 2 * cube_sl, 0..(cube_sl - 1)),
                column: pos.row - 2 * cube_sl,
                dir: Direction::Down
            }
        },
        (_, _) => panic!("Transition from cube face {} with direction {:?} not implemented!", cube_face, pos.dir)
    }
}

fn get_password(pos: &Position) -> usize {
    1000 * (pos.row + 1) + 4 * (pos.column + 1) + match pos.dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3
    }
}
fn main() -> Result<()> {
    let (board, moves) = read_input_file("../inputs/day22_input.txt").unwrap();

    let final_pos = get_final_position(&board, &moves, false);
    println!("Part 1 - Password is: {}", get_password(&final_pos));

    let final_pos = get_final_position(&board, &moves, true);
    println!("Part 2 - Password is: {}", get_password(&final_pos));

    Ok(())
}

fn move_forward_one_step(board: &Board, pos: &Position, cube_overflow: bool) -> Option<Position> {
    let new_pos = get_next_field(board, pos, cube_overflow);

    // Check if new position is empty
    if *board.get(new_pos.row, new_pos.column).unwrap() == Field::Empty {
        Some(new_pos)
    } else {
        None
    }
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<(Board, Vec<Move>)> {
    let input = read_to_string(input_path).expect("Could not read input file!");

    let mut lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    if lines.len() < 3 {
        bail!("Too few lines!")
    }

    // Split off moves input from the board input
    let move_line = lines.pop().unwrap();
    let empty_line = lines.pop().unwrap();
    if !empty_line.is_empty() {
        bail!("Second last line was not empty!");
    }

    // Parse board
    let mut rows: Vec<Vec<Field>> = lines
        .iter()
        .map(|l| {
            l.chars().map(|c| match c {
                '.' => Field::Empty,
                '#' => Field::Blocked,
                ' ' => Field::OffMap,
                _ => panic!("Unknown character!")
            }).collect()
        })
        .collect();
    
    // Make all rows the same length (spaces are right-trimmed in the input file)
    let row_length = rows.iter().map(|r| r.len()).max().unwrap();
    for row in &mut rows {
        row.resize_with(row_length, || Field::OffMap);
    }

    let board = Array2D::from_rows(&rows).expect("Could not build grid!");

    // Parse moves
    let mut moves = vec!();
    let mut cur_move_count_str = String::new();

    for c in move_line.chars() {
        if c.is_ascii_digit() {
            cur_move_count_str.push(c);
        } else {
            if !cur_move_count_str.is_empty() {
                moves.push(Move::Forward { steps: cur_move_count_str.parse().expect("Could not parse number!") });
                cur_move_count_str = String::new();
            }

            match c {
                'L' => moves.push(Move::Turn { turn_right: false }),
                'R' => moves.push(Move::Turn { turn_right: true }),
                _ => bail!("Unknown character found in moves line!")
            };
        }
    }

    if !cur_move_count_str.is_empty() {
        moves.push(Move::Forward { steps: cur_move_count_str.parse().expect("Could not parse number!") });
    }

    Ok((board, moves))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let (board, moves) = read_input_file("../inputs/day22_example.txt").unwrap();
        let final_pos = get_final_position(&board, &moves, false);
        assert_eq!(final_pos.row + 1, 6);
        assert_eq!(final_pos.column + 1, 8);
        assert_eq!(final_pos.dir, Direction::Right);
        assert_eq!(get_password(&final_pos), 6032);
    }

    // Not unit test for part 2, as the cube layout is hard-coded above (how ugly) and the layout differs in the example
}
