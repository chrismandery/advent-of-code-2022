use anyhow::{anyhow, Context, Result};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

type CubePos = (i8, i8, i8);

const CUBE_OFFSETS: [(i8, i8, i8); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1)
];

fn calc_outside_air_cubes(lava_cubes: &HashSet<CubePos>, visited_air_cubes: &mut HashSet<CubePos>, min_coords: &CubePos,
    max_coords: &CubePos, cur_coord: CubePos) {
    // Abort if current position is outside of the min/max coordinates to limit recursion
    if cur_coord.0 < min_coords.0 || cur_coord.1 < min_coords.1 || cur_coord.2 < min_coords.2 ||
        cur_coord.0 > max_coords.0 || cur_coord.1 > max_coords.1 || cur_coord.2 > max_coords.2 {
        return;
    }

    // Only recurse if the current position is neither lava nor a before visited air cube
    if !lava_cubes.contains(&cur_coord) && visited_air_cubes.insert(cur_coord) {
        for offset in CUBE_OFFSETS {
            calc_outside_air_cubes(lava_cubes, visited_air_cubes, min_coords, max_coords,
                (cur_coord.0 + offset.0, cur_coord.1 + offset.1, cur_coord.2 + offset.2))
        }
    }
}

/// The second parameter is used only during second part of the puzzle.
fn calc_surface_area_part1(cubes: &HashSet<CubePos>, outside_air_cubes: Option<&HashSet<CubePos>> ) -> usize {
    let mut surface_count = 0;

    for c in cubes {
        for offset in CUBE_OFFSETS {
            let neighbor = (c.0 + offset.0, c.1 + offset.1, c.2 + offset.2);
            if !cubes.contains(&neighbor) && outside_air_cubes.map(|oac| oac.contains(&neighbor)).unwrap_or(true) {
                surface_count += 1;
            }
        }
    }

    surface_count
}

/// For the second part of the puzzle (considering only the real surface area without counting cubes of trapped air), we have to be a bit
/// more sophisticated. Here, we are first using a BFS to determine which (air) cubes are actually part of the "outside air".
fn calc_surface_area_part2(cubes: &HashSet<CubePos>) -> usize {
    // Determine min and max coordinates for the BFS (min/max may be different on each axis)
    let min_coords = (
        cubes.iter().map(|c| c.0).min().unwrap() - 1,
        cubes.iter().map(|c| c.1).min().unwrap() - 1,
        cubes.iter().map(|c| c.2).min().unwrap() - 1
    );
    let max_coords = (
        cubes.iter().map(|c| c.0).max().unwrap() + 1,
        cubes.iter().map(|c| c.1).max().unwrap() + 1,
        cubes.iter().map(|c| c.2).max().unwrap() + 1
    );

    let mut outside_air = HashSet::new();
    calc_outside_air_cubes(&cubes, &mut outside_air, &min_coords, &max_coords, min_coords);

    calc_surface_area_part1(&cubes, Some(&outside_air))
}

fn main() -> Result<()> {
    let cubes = read_input_file("../inputs/day18_input.txt")?;
    println!("Total surface area (part 1, including trapped air): {}", calc_surface_area_part1(&cubes, None));
    println!("Total surface area (part 2, using BFS to only consider actual surface) {}", calc_surface_area_part2(&cubes));

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<HashSet<CubePos>> {
    let input = read_to_string(input_path).context("Could not read input file!")?;
    let res = input
        .lines()
        .map(|l| {
            let coords: Vec<i8> = l.split(',').map(|s| s.parse().expect("Could not parse number!")).collect();
            if coords.len() == 3 {
                Ok((coords[0], coords[1], coords[2]))
            } else {
                Err(anyhow!("Three-dimensional coordinates expected!"))
            }
        })
        .collect();

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let cubes = read_input_file("../inputs/day18_example.txt").unwrap();
        assert_eq!(calc_surface_area_part1(&cubes, None), 64);
        assert_eq!(calc_surface_area_part2(&cubes), 58);
    }
}
