use anyhow::{bail, Context, Result};
use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
enum Operator {
    Add,
    Mul
}

#[derive(Debug)]
enum Operand {
    Old,
    Constant { val: u64 }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,  // Needs to be u64 for the first part of the puzzle already (u32 overflows during multiplication)
    operator: Operator,
    operand: Operand,
    divisor: u64,
    target_true: usize,
    target_false: usize,
    items_inspected_count: u64
}

fn get_level_of_monkey_business(monkeys: &[Monkey]) -> u64 {
    let mut inspection_counts: Vec<u64> = monkeys.iter().map(|m| m.items_inspected_count).collect();
    inspection_counts.sort_unstable();
    inspection_counts.reverse();
    inspection_counts[0] * inspection_counts[1]
}

fn main() -> Result<()> {
    // First part of the puzzle
    let mut monkeys = read_input_file("../inputs/day11_input.txt")?;

    for _ in 0..20 {
        simulate_round(&mut monkeys, true).unwrap();
    }

    println!("First part - resulting level of monkey business: {}", get_level_of_monkey_business(&monkeys));

    // Second part of the puzzle
    let mut monkeys = read_input_file("../inputs/day11_input.txt")?;

    for _ in 0..10000 {
        simulate_round(&mut monkeys, false).unwrap();
    }

    println!("Second part - resulting level of monkey business: {}", get_level_of_monkey_business(&monkeys));

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<Monkey>> {
    let re = Regex::new(r"^Monkey (?P<monkey_num>\d+):
  Starting items: (?P<starting_items>[\d ,]+)
  Operation: new = old (?P<operator>[+*]) (?P<operand>(old|\d+))
  Test: divisible by (?P<divisor>\d+)
    If true: throw to monkey (?P<target_true>\d+)
    If false: throw to monkey (?P<target_false>\d+)
$")?;

    let input = read_to_string(input_path).context("Could not read input file!")?;
    let mut res = vec!();
    let mut cur_monkey_num = 0;

    // Read input in chunks of 7 lines
    for chunk in &input.lines().chunks(7) {
        let monkey_description = chunk.collect::<Vec<&str>>().join("\n");
        if let Some(caps) = re.captures(&monkey_description) {
            let monkey_num: i32 = caps.name("monkey_num").unwrap().as_str().parse().context("Could not parse number!")?;
            if monkey_num != cur_monkey_num {
                bail!("Monkeys numbered wrongly! Expected number {}, but was {}.", cur_monkey_num, monkey_num);
            }

            let monkey = Monkey {
                items: caps
                    .name("starting_items")
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|s| s.parse().expect("Could not parse number!"))
                    .collect(),
                operator: match caps.name("operator").unwrap().as_str() {
                    "+" => Operator::Add,
                    "*" => Operator::Mul,
                    _ => bail!("Unsupported operator!")
                },
                operand: match caps.name("operand").unwrap().as_str() {
                    "old" => Operand::Old,
                    val_str => Operand::Constant { val: val_str.parse().context("Could not parse number!")? }
                },
                divisor: caps.name("divisor").unwrap().as_str().parse().context("Could not parse number!")?,
                target_true: caps.name("target_true").unwrap().as_str().parse().context("Could not parse number!")?,
                target_false: caps.name("target_false").unwrap().as_str().parse().context("Could not parse number!")?,
                items_inspected_count: 0
            };

            res.push(monkey);
            cur_monkey_num += 1;
        } else {
            bail!("Could not parse monkey description:\n{}", monkey_description);
        }
    }

    Ok(res)
}

/// Monkey vector is altered with the changes made during this round.
fn simulate_round(monkeys: &mut [Monkey], enable_divide_by_three: bool) -> Result<()> {
    // Calculate least common multiple of all divisors: Necessary for numerical optimization for the second part of the puzzle (see below)
    let all_divisors_lcm = monkeys.iter().map(|m| m.divisor).reduce(|x, y| lcm(x, y)).unwrap();

    for i in 0..monkeys.len() {
        while let Some(mut worry) = monkeys[i].items.pop_front() {
            // Apply operation for this monkey
            match &monkeys[i].operator {
                Operator::Add => {
                    match monkeys[i].operand {
                        Operand::Old => worry += worry,
                        Operand::Constant { val } => worry += val
                    }
                }
                Operator::Mul => {
                    match &monkeys[i].operand {
                        Operand::Old => worry *= worry,
                        Operand::Constant { val } => worry *= val
                    }
                }
            }

            if enable_divide_by_three {
                // Post-inspection division by three
                worry /= 3;
            } else {
                // Prevent numbers from getting to large (for second part of the puzzle)
                // We are running all calculations in the space modulo-X (where X should be the least common multiple of all divisors)
                worry %= all_divisors_lcm;
            }

            // Evaluate condition
            let is_divisible = worry % monkeys[i].divisor == 0;

            // Pass item to target monkey
            let target_monkey = if is_divisible { monkeys[i].target_true } else { monkeys[i].target_false };
            monkeys[target_monkey].items.push_back(worry);

            // Increase item inspection count
            monkeys[i].items_inspected_count += 1;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut monkeys = read_input_file("../inputs/day11_example.txt").unwrap();

        // Simulate first round and check items
        simulate_round(&mut monkeys, true).unwrap();
        assert_eq!(monkeys[0].items, [20, 23, 27, 26]);
        assert_eq!(monkeys[1].items, [2080, 25, 167, 207, 401, 1046]);
        assert!(monkeys[2].items.is_empty());
        assert!(monkeys[3].items.is_empty());

        // Simulate 19 more rounds and check inspection counts
        for _ in 0..19 {
            simulate_round(&mut monkeys, true).unwrap();
        }
        assert_eq!(monkeys[0].items_inspected_count, 101);
        assert_eq!(monkeys[1].items_inspected_count, 95);
        assert_eq!(monkeys[2].items_inspected_count, 7);
        assert_eq!(monkeys[3].items_inspected_count, 105);
    }

    #[test]
    fn example2() {
        let mut monkeys = read_input_file("../inputs/day11_example.txt").unwrap();

        for _ in 0..10000 {
            simulate_round(&mut monkeys, false).unwrap();
        }
        assert_eq!(monkeys[0].items_inspected_count, 52166);
        assert_eq!(monkeys[1].items_inspected_count, 47830);
        assert_eq!(monkeys[2].items_inspected_count, 1938);
        assert_eq!(monkeys[3].items_inspected_count, 52013);
    }
}
