use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

type Cargo = char;
type CargoStack = Vec<Cargo>;

struct CargoTransfer {
    from: usize,
    to: usize,
    amount: usize
}

struct Input {
    start_configuration: Vec<CargoStack>,
    instructions: Vec<CargoTransfer>
}

enum ParserState {
    StartConfiguration,
    EmptyLine,
    CargoTransfer
}

fn calc_final_configuration(input: &Input, move_all_crates_at_once: bool) -> Result<Vec<CargoStack>> {
    let mut stacks = input.start_configuration.clone();

    for instr in input.instructions.iter() {
        if move_all_crates_at_once {
            let from_len = stacks[instr.from - 1].len();
            let mut transfer_elements = stacks[instr.from - 1].split_off(from_len - instr.amount);
            stacks[instr.to - 1].append(&mut transfer_elements);
        } else {
            for _ in 0..instr.amount {
                let c = stacks[instr.from - 1].pop().context("Stack ran empty during cargo transfer!")?;
                stacks[instr.to - 1].push(c);
            }
        }
    }

    Ok(stacks)
}

fn get_top_elements(config: &Vec<CargoStack>) -> Result<String> {
    let mut res = String::new();

    for cs in config {
        let c = cs.last().ok_or_else(|| anyhow!("Cargo stack is empty in the final configuration!"))?;
        res.push(*c);
    }

    Ok(res)
}

fn main() -> Result<()> {
    let input = read_input_file("../inputs/day5_input.txt")?;

    let final_config = calc_final_configuration(&input, false)?;
    println!("Solution for first part is: {}", get_top_elements(&final_config)?);

    let final_config = calc_final_configuration(&input, true)?;
    println!("Solution for second part is: {}", get_top_elements(&final_config)?);

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Input> {
    // We are assuming that the stack numbers are just one-digit numbers here (which is true for the provided input)
    let re_start_configuration = Regex::new(r"\[[A-Z]\]|( {3,4})")?;
    let re_stack_numbers = Regex::new(r" \d ")?;
    let re_cargo_transfer = Regex::new(r"^move (?P<amount>\d+) from (?P<from>\d) to (?P<to>\d)$")?;

    let input = read_to_string(input_path)?;
    let lines = input.lines();
    let mut state = ParserState::StartConfiguration;
    let mut config = vec!();
    let mut instr = vec!();

    for l in lines {
        match state {
            ParserState::StartConfiguration => {
                // Check if this line still contains a cargo
                if l.contains('[') {
                    // Collect all matches (cargo or empty field) into a vector
                    let cap: Vec<&str> = re_start_configuration
                        .find_iter(l)
                        .map(|m| m.as_str())
                        .collect();

                    // Check number of stacks that we have read in this line
                    let stack_count = cap.len();
                    if config.is_empty() {
                        config = vec![vec!(); stack_count];  // Create initial vector
                    } else if config.len() != stack_count {
                        bail!("We have already started reading the initial configuration, but now a line has {} instead of the initially set up {} stacks. Check the input.",
                            stack_count, config.len());
                    }

                    for i in 0..stack_count {
                        let c = cap[i];
                        if !c.trim().is_empty() {
                            if c.len() != 3 {
                                bail!("Cargo match does not follow [x] format!");
                            }

                            // Inserting at the front is a terribly slow operation, but here it should be okay since the input is quite small (< 10 lines)
                            config[i].insert(0, c.chars().nth(1).unwrap());
                        }
                    }
                } else {
                    // The input is not a valid "initial configuration line" anymore, so now we are expecting the numbers of the stacks in the input
                    let stack_count = re_stack_numbers.find_iter(l).count();
                    if stack_count != config.len() {
                        bail!("Done with reading the initial configuration, but the numbering of the cargo stacks ({}) in the input does not match the number of stacks we have read ({}).",
                            stack_count, config.len());
                    }

                    state = ParserState::EmptyLine;
                }
            },
            ParserState::EmptyLine => {
                if !l.is_empty() {
                    bail!("Expected an empty line after the stack numbers line!");
                }

                state = ParserState::CargoTransfer;
            },
            ParserState::CargoTransfer => {
                let caps = re_cargo_transfer.captures(l).context("Could not parse cargo transfer line!")?;
                instr.push(CargoTransfer {
                    from: caps.name("from").unwrap().as_str().parse().context("Could not parse \"from\" number!")?,
                    to: caps.name("to").unwrap().as_str().parse().context("Could not parse \"to\" number!")?,
                    amount: caps.name("amount").unwrap().as_str().parse().context("Could not parse \"amount\" number!")?
                });
            }
        }
    }

    Ok(Input {
        start_configuration: config,
        instructions: instr
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = read_input_file("../inputs/day5_example.txt").unwrap();

        let final_config = calc_final_configuration(&input, false).unwrap();
        assert_eq!(get_top_elements(&final_config).unwrap(), "CMZ");

        let final_config = calc_final_configuration(&input, true).unwrap();
        assert_eq!(get_top_elements(&final_config).unwrap(), "MCD");
    }
}
