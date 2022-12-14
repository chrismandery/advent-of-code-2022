use anyhow::Result;
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

struct CleaningRange {
    start: u32,
    end: u32
}

impl CleaningRange {
    fn contains(&self, other: &CleaningRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &CleaningRange) -> bool {
        // Beside the case that other strictly contains self, we just need to check two cases:
        // Is other's start is within self or is other's end within self
        other.contains(&self) ||
        (other.start >= self.start && other.start <= self.end) ||
        (other.end >= self.start && other.end <= self.end)
    }
}

struct CleaningPair {
    r1: CleaningRange,
    r2: CleaningRange
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<CleaningPair>> {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    let input = read_to_string(input_path)?;
    let res = input
        .lines()
        .map(|l| {
            let cap = re.captures(&l).expect("Could not parse input line!");
            let nums: Vec<u32> = (1..=4).map(|n| {
                cap
                    .get(n)
                    .expect("Could not get capture group from regex!")
                    .as_str()
                    .parse()
                    .expect("Could not parse number!")
            }).collect();

            CleaningPair {
                r1: CleaningRange {
                    start: nums[0],
                    end: nums[1],
                },
                r2: CleaningRange {
                    start: nums[2],
                    end: nums[3]
                }
            }
        })
        .collect();

    Ok(res)
}

fn main() {
    let cps = read_input_file("../inputs/day4_input.txt").unwrap();

    let count_contains = cps
        .iter()
        .filter(|cp| cp.r1.contains(&cp.r2) || cp.r2.contains(&cp.r1))
        .count();
    println!("Number of assignment pairs where one fully contains the other: {}", count_contains);

    let count_overlaps = cps
        .iter()
        .filter(|cp| cp.r1.overlaps(&cp.r2))
        .count();
    println!("Number of assignment pairs that overlap: {}", count_overlaps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let cps = read_input_file("../inputs/day4_example.txt").unwrap();

        let count_contains = cps
            .iter()
            .filter(|cp| cp.r1.contains(&cp.r2) || cp.r2.contains(&cp.r1))
            .count();
        assert_eq!(count_contains, 2);

        let count_overlaps = cps
            .iter()
            .filter(|cp| cp.r1.overlaps(&cp.r2))
            .count();
        assert_eq!(count_overlaps, 4);
    }
}
