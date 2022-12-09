use anyhow::Result;
use array_tool::vec::Intersect;
use itertools::Itertools;
use std::fs::read_to_string;
use std::path::Path;

struct Rucksack {
    all: Vec<char>,
    comp1: Vec<char>,
    comp2: Vec<char>
}

impl Rucksack {
    fn get_duplicated_priorities_sum(&self) -> u32 {
        let common_elements = self.comp1.intersect(self.comp2.clone());
        common_elements.into_iter().map(|c| get_char_priority(c)).sum()
    }
}

fn find_common_badge_priority(r1: &Rucksack, r2: &Rucksack, r3: &Rucksack) -> u32 {
    let badge_chars = r1.all.intersect(r2.all.clone()).intersect(r3.all.clone());
    if badge_chars.len() != 1 {
        panic!("No badge char (common char in three subsequent lines) found for group!");
    }

    get_char_priority(badge_chars.first().unwrap().clone())
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
                all: l.chars().collect(),
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

    let badge_priority_sum: u32 = rucksack_list
        .iter()
        .tuples::<(_, _, _)>()
        .map(|(r1, r2, r3)| {
            find_common_badge_priority(r1, r2, r3)
        })
        .sum();
    println!("Sum of badge priorities is: {}", badge_priority_sum);
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

        let badge_priority_sum: u32 = rucksack_list
            .iter()
            .tuples::<(_, _, _)>()
            .map(|(r1, r2, r3)| {
                find_common_badge_priority(r1, r2, r3)
            })
            .sum();

        assert_eq!(badge_priority_sum, 70);
    }
}
