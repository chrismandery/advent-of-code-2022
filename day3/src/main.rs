use anyhow::Result;
use array_tool::vec::Intersect;
use std::fs::read_to_string;
use std::path::Path;

struct Rucksack {
    comp1: Vec<char>,
    comp2: Vec<char>
}

impl Rucksack {
    fn get_duplicated_priorities_sum(&self) -> u32 {
        let common_elements = self.comp1.intersect(self.comp2.clone());  // TODO remove clone
        common_elements.into_iter().map(|c| get_char_priority(c)).sum()
    }
}

fn get_char_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else if c.is_ascii_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        panic!("Non-letter character not supported!");
    }
}

fn read_rucksack_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<Rucksack>> {
    let input = read_to_string(input_path)?;
    let rucksack_list = input
        .lines()
        .map(|l| {
            if l.len() % 2 != 0 {
                panic!("Input line has odd length, this must never happen.");
            }

            let (l1, l2) = l.split_at(l.len() / 2);

            Rucksack {
                comp1: l1.chars().collect(),
                comp2: l2.chars().collect()
            }
        })
        .collect();

    Ok(rucksack_list)
}

fn main() {
    let rucksack_list = read_rucksack_file("../inputs/day3_input.txt").unwrap();
    let sum: u32 = rucksack_list
        .iter()
        .map(|r| r.get_duplicated_priorities_sum())
        .sum();
    println!("Sum of priorities is: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let rucksack_list = read_rucksack_file("../inputs/day3_example.txt").unwrap();
        let sum: u32 = rucksack_list
            .iter()
            .map(|r| r.get_duplicated_priorities_sum())
            .sum();

        assert_eq!(sum, 157);
    }
}