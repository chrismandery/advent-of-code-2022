use anyhow::{bail, Result};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
enum HandChoice {
    Rock,
    Paper,
    Scissors
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
    fn decode_input_line(line: &str) -> Result<Self> {
        if line.len() != 3 {
            bail!("Input line has invalid length: {}", line);
        }

        let mut chars = line.chars();
        let m = Match {
            opponent_choice: HandChoice::decode_char(chars.nth(0).unwrap())?,
            own_choice: HandChoice::decode_char(chars.nth(1).unwrap())?
        };

        Ok(m)
    }

    fn get_match_points(&self) -> usize {
        if self.own_choice == self.opponent_choice {
            3  // Draw
        } else if
            (self.own_choice == HandChoice::Rock && self.opponent_choice == HandChoice::Scissors) ||
            (self.own_choice == HandChoice::Paper && self.opponent_choice == HandChoice::Rock) ||
            (self.own_choice == HandChoice::Scissors && self.opponent_choice == HandChoice::Paper) {
            6  // Win
        } else {
            0  // Loss
        }
    }
}

fn get_matches_from_input<P: AsRef<Path>>(input_path: P) -> Result<Vec<Match>> {
    let input = read_to_string(input_path)?;
    let matches = input
        .lines()
        .map(|l| Match::decode_input_line(l).expect("Cannot decode input line!"))
        .collect();

    Ok(matches)
}

fn main() -> Result<()> {
    let matches = get_matches_from_input("../inputs/day2_input.txt")?;
    let points: usize = matches
        .iter()
        .map(|m| m.own_choice.get_hand_points() + m.get_match_points())
        .sum();
    println!("Total points for {} matches: {}", matches.len(), points);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_example() {
        let matches = get_matches_from_input("../inputs/day2_example.txt").unwrap();
        dbg!(&matches);
        let points: usize = matches
            .iter()
            .map(|m| m.own_choice.get_hand_points() + m.get_match_points())
            .sum();
        assert_eq!(points, 15);
    }
}
