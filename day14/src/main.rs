use anyhow::{bail, Context, Result};
use regex::Regex;
use std::cmp::{min, max};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

struct Field {
    /// We do not actually need to store the field as an array, we can just keep a set of the occupied fields
    occupied_fields: HashSet<(u32, u32)>,

    /// If a falling sand unit exceeds the maximum depth, we can assume it has fallen into the abyss
    max_depth: u32
}

/// add_virtual_floor controls whether there is a virtual floor at max_depth + 2 for the second part of the puzzle.
fn calc_fallen_sand_count(f: &mut Field, add_virtual_floor: bool) -> Result<u32> {
    let mut count = 0;

    while simulate_sand_unit(f, add_virtual_floor)? {
        count += 1;
    }

    Ok(count)
}

fn main() -> Result<()> {
    let mut field = read_input_file("../inputs/day14_input.txt")?;
    println!("Units of send that have come to rest (first part): {}", calc_fallen_sand_count(&mut field, false)?);

    let mut field = read_input_file("../inputs/day14_input.txt")?;
    println!("Units of send that have come to rest (second part): {}", calc_fallen_sand_count(&mut field, true)?);

    Ok(())
}

fn parse_coord(s: &str) -> Result<(u32, u32)> {
    let mut it = s.split(",");

    let x = it
        .next()
        .with_context(|| format!("Could not extract X: {}", s))?
        .parse().context("Could not parse number!")?;
    let y = it
        .next()
        .with_context(|| format!("Could not extract Y: {}", s))?
        .parse().context("Could not parse number!")?;

    if it.next().is_some() {
        bail!("Unexpected third dimension: {}", s);
    }

    Ok((x, y))
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Field> {
    let input = read_to_string(input_path).context("Could not read input file!")?;
    let re_split = Regex::new(" -> ")?;

    let mut occupied_fields = HashSet::new();
    let mut max_depth = 0;

    for line in input.lines() {
        let mut coord_strings = re_split.split(line);
        let mut cur_coord = parse_coord(coord_strings.next().expect("Could not extract first coordinate?!"))?;

        for next_coord_str in coord_strings {
            let next_coord = parse_coord(next_coord_str)?;

            // Mark all fields from the current to the next coordinate as occupied
            if cur_coord.0 == next_coord.0 {
                let max_y = max(cur_coord.1, next_coord.1);

                for y in min(cur_coord.1, next_coord.1)..=max_y {
                    occupied_fields.insert((cur_coord.0, y));
                }

                if max_y > max_depth {
                    max_depth = max_y;
                }
            } else if cur_coord.1 == next_coord.1 {
                for x in min(cur_coord.0, next_coord.0)..=max(cur_coord.0, next_coord.0) {
                    occupied_fields.insert((x, cur_coord.1));
                }

                if cur_coord.1 > max_depth {
                    max_depth = cur_coord.1;
                }
            } else {
                bail!("Connection between {:?} and {:?} is neither horizontal nor vertical!", cur_coord, next_coord);
            }

            cur_coord = next_coord;
        }
    }

    Ok(Field {
        occupied_fields,
        max_depth
    })
}

/// Simulates one unit of sand falling, adding it to the field (if it comes to rest on the field). Returns whether the unit of sand come to
/// rest on the field (true) or whether it has fallen in the abyss (return value false, without virtual floor resp. first part of the
/// puzzle) or whether the spawn point is already blocked (return value false, with virtual floor resp. second part of the puzzle).
fn simulate_sand_unit(f: &mut Field, add_virtual_floor: bool) -> Result<bool> {
    let mut cur_pos = (500u32, 0u32);

    // Check if spawn position is already blocked
    if f.occupied_fields.contains(&cur_pos) {
        return Ok(false);
    }

    while add_virtual_floor || cur_pos.1 < f.max_depth {
        // Check if any of the fields below the sand unit is empty (if so, move there)
        if add_virtual_floor && cur_pos.1 == f.max_depth + 1 {
            // Sand unit cannot move down anymore and comes to rest on the floor
            f.occupied_fields.insert(cur_pos);
            return Ok(true);
        } else if !f.occupied_fields.contains(&(cur_pos.0, cur_pos.1 + 1)) {
            cur_pos.1 += 1;
        } else if !f.occupied_fields.contains(&(cur_pos.0 - 1, cur_pos.1 + 1)) {
            cur_pos.0 -= 1;
            cur_pos.1 += 1;
        } else if !f.occupied_fields.contains(&(cur_pos.0 + 1, cur_pos.1 + 1)) {
            cur_pos.0 += 1;
            cur_pos.1 += 1;
        } else {
            // Sand unit cannot move anywhere and comes to rest - check if sand unit is now blocking the spawn position
            f.occupied_fields.insert(cur_pos);
            return Ok(true);
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let mut field = read_input_file("../inputs/day14_example.txt").unwrap();
        assert_eq!(calc_fallen_sand_count(&mut field, false).unwrap(), 24);
    }

    #[test]
    fn example_part2() {
        let mut field = read_input_file("../inputs/day14_example.txt").unwrap();
        assert_eq!(calc_fallen_sand_count(&mut field, true).unwrap(), 93);
    }
}
