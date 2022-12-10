use anyhow::{bail, Context, Result};
use std::fs::read_to_string;
use std::path::Path;

enum Instruction {
    Noop,
    Add { value: i32 }  // Only takes one cycle for execution (addx in the input is translated to a pair of Noop+Add)
}

fn main() -> Result<()> {
    let instructions = read_input_file("../inputs/day10_input.txt")?;
    let signal_strength = process_instructions(&instructions);
    println!("Sum of signal strengtes is: {}", signal_strength);

    Ok(())
}

fn process_instructions(instructions: &[Instruction]) -> i32 {
    let mut x = 1;
    let mut cur_cycle = 0;
    let mut signal_strength_sum = 0;
    let mut cur_display_line = String::new();

    for i in instructions {
        let sprite_position: i32 = cur_cycle % 40;
        cur_display_line.push(if (sprite_position - x).abs() <= 1 { '#' } else { '.' });

        if cur_display_line.len() == 40 {
            println!("{}", cur_display_line);  // Pass --nocapture to see this also for the unit test
            cur_display_line = String::new();
        }

        cur_cycle += 1;

        if cur_cycle == 20 || (cur_cycle - 20) % 40 == 0 {
            signal_strength_sum += x * cur_cycle;
        }

        match i {
            Instruction::Noop => {},
            Instruction::Add { value } => {
                x += value;
            }
        }
    }

    signal_strength_sum
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<Instruction>> {
    let input = read_to_string(input_path).context("Could not read input file!")?;
    let mut res = vec!();

    for l in input.lines() {
        if l == "noop" {
            res.push(Instruction::Noop);
        } else if l.starts_with("addx ") {
            // Translate addx command to a pair of Noop and Add instructions (both of which take one cycle for execution)
            res.push(Instruction::Noop);
            res.push(Instruction::Add { value: l[5..].parse().context("Could not parse number!")? });
        } else {
            bail!("Could not parse line: {}", l);
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instructions = read_input_file("../inputs/day10_example.txt").unwrap();
        let signal_strength = process_instructions(&instructions);
        assert_eq!(signal_strength, 13140);
    }
}
