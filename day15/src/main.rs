use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::cmp::max;
use std::fs::read_to_string;
use std::ops::Sub;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
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
    closest_beacon: Position,
    closest_beacon_dist: u64
}

fn calc_positions_without_beacon(sensors: &[Sensor], row_y: i64) -> usize {
    // Determine start and end position for scanning X positions
    let start_x = sensors.iter().map(|sensor| sensor.position.0 as i64 - sensor.closest_beacon_dist as i64).min().unwrap();
    let end_x = sensors.iter().map(|sensor| sensor.position.0 as i64 + sensor.closest_beacon_dist as i64).max().unwrap();

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

fn find_missing_beacon(sensors: &[Sensor], max_coord: i64) -> Result<Position> {
    for x in 0..=max_coord {
        let mut y = 0;

        while y <= max_coord {
            let cur_pos = Position(x, y);

            // Check why no beacon can exist at this position: Calculate for each sensors how much closer we are to the sensor than the
            // sensor's closest beacon
            let closer_to_sensor_than_closest_beacon = sensors.iter().filter_map(|sensor| {
                let dist_to_sensor = &cur_pos - &sensor.position;

                if dist_to_sensor <= sensor.closest_beacon_dist {
                    Some(sensor.closest_beacon_dist - dist_to_sensor)
                } else {
                    None
                }
            }).max();

            if let Some(dist) = closer_to_sensor_than_closest_beacon {
                // Skip this many fields: If we are, e.g., 5 units closer to a sensor than its closest beacon, none of the next five fields can
                // contain a beacon (otherwise, it would be the closest beacon to this sensor)
                y += max(dist as i64, 1);
            } else {
                // We found the missing beacon
                return Ok(Position(x, y));
            }
        }
    }

    Err(anyhow!("No beacon found!"))
}

fn main() -> Result<()> {
    let sensors = read_input_file("../inputs/day15_input.txt")?;
    println!("Positions without beacon in row y=2000000: {}", calc_positions_without_beacon(&sensors, 2000000));
    println!("Position of missing (distress) beacon: {:?}", find_missing_beacon(&sensors, 4000000)?);

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
        assert_eq!(find_missing_beacon(&sensors, 20).unwrap(), Position(14, 11));
    }
}
