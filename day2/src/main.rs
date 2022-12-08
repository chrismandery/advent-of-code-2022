use anyhow::{bail, Result};
use std::convert::TryFrom;
use std::fs::read_to_string;
use std::path::Path;

/// Following the numbering of the enum variants, x will beat y exactly when (x - y) % 3 == 1
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HandChoice {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}

impl TryFrom<isize> for HandChoice {
    type Error = ();

    fn try_from(v: isize) -> Result<Self, Self::Error> {
        match v {
            x if x == HandChoice::Rock as isize => Ok(HandChoice::Rock),
            x if x == HandChoice::Paper as isize => Ok(HandChoice::Paper),
            x if x == HandChoice::Scissors as isize => Ok(HandChoice::Scissors),
            _ => Err(()),
        }
    }
}

impl HandChoice {
    fn decode_char(c: char) -> Result<Self> {
        let res = match c {
            'A' => HandChoice::Rock,
            'B' => HandChoice::Paper,
            'C' => HandChoice::Scissors,
            'X' => HandChoice::Rock,
            'Y' => HandChoice::Paper,
            'Z' => HandChoice::Scissors,
            _ => bail!("Invalid hand symbol: {}", c)
        };

        Ok(res)
    }

    fn get_hand_points(&self) -> usize {
        match self {
            HandChoice::Rock => 1,
            HandChoice::Paper => 2,
            HandChoice::Scissors => 3
        }
    }
}

#[derive(Debug)]
struct Match {
    opponent_choice: HandChoice,
    own_choice: HandChoice
}

impl Match {
    fn decode_input_line(line: &str, second_half_of_puzzle: bool) -> Result<Self> {
        if line.len() != 3 {
            bail!("Input line has invalid length: {}", line);
        }

        let mut chars = line.chars();
        let opponent_choice = HandChoice::decode_char(chars.nth(0).unwrap())?;
        let m = Match {
            opponent_choice: opponent_choice,
            own_choice: {
                let c = chars.nth(1).unwrap();
                if second_half_of_puzzle {
                    match c {
                        'X' => ((opponent_choice as isize - 1).rem_euclid(3)).try_into().unwrap(),  // Own hand should lose
                        'Y' => opponent_choice,                                                     // Draw
                        'Z' => ((opponent_choice as isize + 1).rem_euclid(3)).try_into().unwrap(),  // Own hand should win
                        _ => bail!("Invalid hand symbol: {}", c)
                    }
                } else {
                    HandChoice::decode_char(c)?
                }
            }
        };

        Ok(m)
    }

    fn get_match_points(&self) -> usize {
        if self.own_choice == self.opponent_choice {
            3  // Draw
        } else if (self.own_choice as isize - self.opponent_choice as isize).rem_euclid(3) == 1 {
            6  // Win
        } else {
            0  // Loss
        }
    }
}

/// If second_half_of_puzzle is true, the second symbol does not represent the own hand, but the desired outcome of the match
/// (the own hand symbol must then be determined from the opponent's hand and the desired outcome)
fn get_matches_from_input<P: AsRef<Path>>(input_path: P, second_half_of_puzzle: bool) -> Result<Vec<Match>> {
    let input = read_to_string(input_path)?;
    let matches = input
        .lines()
        .map(|l| Match::decode_input_line(l, second_half_of_puzzle).expect("Cannot decode input line!"))
        .collect();

    Ok(matches)
}

fn main() -> Result<()> {
    // First half of puzzle
    let matches = get_matches_from_input("../inputs/day2_input.txt", false)?;
    let points: usize = matches
        .iter()
        .map(|m| m.own_choice.get_hand_points() + m.get_match_points())
        .sum();
    println!("First half - total points for {} matches: {}", matches.len(), points);

    // Second half of puzzle
    let matches = get_matches_from_input("../inputs/day2_input.txt", true)?;
    let points: usize = matches
        .iter()
        .map(|m| m.own_choice.get_hand_points() + m.get_match_points())
        .sum();
    println!("Second half - total points for {} matches: {}", matches.len(), points);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_half() {
        let matches = get_matches_from_input("../inputs/day2_example.txt", false).unwrap();
        let points: usize = matches
            .iter()
            .map(|m| m.own_choice.get_hand_points() + m.get_match_points())
            .sum();
        assert_eq!(points, 15);
    }

    #[test]
    fn example_second_half() {
        let matches = get_matches_from_input("../inputs/day2_example.txt", true).unwrap();
        let points: usize = matches
            .iter()
            .map(|m| m.own_choice.get_hand_points() + m.get_match_points())
            .sum();
        assert_eq!(points, 12);
    }
}
