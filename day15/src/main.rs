use anyhow::{Context, Result};
use regex::Regex;
use std::fs::read_to_string;
use std::ops::Sub;
use std::path::Path;

#[derive(Clone, PartialEq)]
struct Position(i64, i64);

impl Sub for &Position {
    type Output = u64;

    // Manhattan distance
    fn sub(self, other: Self) -> Self::Output {
        (self.0 - other.0).abs() as u64 + (self.1 - other.1).abs() as u64
    }
}

struct Sensor {
    position: Position,
    closest_beacon: Position,  // TODO not used
    closest_beacon_dist: u64
}

fn calc_positions_without_beacon(sensors: &[Sensor], row_y: i64) -> usize {
    // Determine start and end position for scanning X positions
    let start_x = sensors.iter().map(|sensor| sensor.position.0 as i64 - sensor.closest_beacon_dist as i64).min().unwrap();
    let end_x = sensors.iter().map(|sensor| sensor.position.0 as i64 + sensor.closest_beacon_dist as i64).max().unwrap();

    dbg!(&start_x);
    dbg!(&end_x);

    (start_x..=end_x).filter(|x| {
        let cur_pos = Position(*x, row_y);

        let is_beacon = sensors.iter().any(|sensor| {
            cur_pos == sensor.closest_beacon
        });

        // Loop over all sensors and check whether we are closer to a sensor than its closest_beacon_dist (which means that our current
        // position cannot possibly contain a beacon)
        let cannot_be_other_beacon = !is_beacon && sensors.iter().any(|sensor| {
            &cur_pos - &sensor.position <= sensor.closest_beacon_dist
        });

        // println!("X = {} => is_beacon = {} / cannot_be_other_beacon = {}", x, is_beacon, cannot_be_other_beacon);
        cannot_be_other_beacon
    }).count()
}

fn main() -> Result<()> {
    let sensors = read_input_file("../inputs/day15_input.txt")?;
    println!("Positions without beacon in row y=2000000: {}", calc_positions_without_beacon(&sensors, 2000000));

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<Sensor>> {
    let re = Regex::new(r"Sensor at x=(?P<sensor_x>[-\d]+), y=(?P<sensor_y>[-\d]+): closest beacon is at x=(?P<beacon_x>[-\d]+), y=(?P<beacon_y>[-\d]+)")?;

    let input = read_to_string(input_path).context("Could not read input file!")?;
    let res = input
        .lines()
        .map(|l| {
            let caps = re.captures(&l).expect("Could not parse input line!");

            let position = Position(
                caps.name("sensor_x").unwrap().as_str().parse().unwrap(),
                caps.name("sensor_y").unwrap().as_str().parse().unwrap()
            );
            let closest_beacon = Position(
                caps.name("beacon_x").unwrap().as_str().parse().unwrap(),
                caps.name("beacon_y").unwrap().as_str().parse().unwrap()
            );
            let closest_beacon_dist = &closest_beacon - &position;

            Sensor {
                position,
                closest_beacon,
                closest_beacon_dist
            }
        })
        .collect();
    
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let sensors = read_input_file("../inputs/day15_example.txt").unwrap();
        assert_eq!(calc_positions_without_beacon(&sensors, 10), 26);
    }
}
