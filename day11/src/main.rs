use anyhow::{bail, Context, Result};
use itertools::Itertools;
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
    items: VecDeque<u64>,  // Needs to be u64, u32 overflows during multiplication
    operator: Operator,
    operand: Operand,
    divisor: u64,
    target_true: usize,
    target_false: usize,
    items_inspected_count: usize
}

fn main() -> Result<()> {
    let mut monkeys = read_input_file("../inputs/day11_input.txt")?;

    for _ in 0..20 {
        simulate_round(&mut monkeys).unwrap();
    }

    for m in monkeys.iter() {
        println!("Monkey inspection count: {}", m.items_inspected_count);
    }

    let mut inspection_counts: Vec<usize> = monkeys.iter().map(|m| m.items_inspected_count).collect();
    inspection_counts.sort_unstable();
    inspection_counts.reverse();
    println!("Resulting level of monkey business: {}", inspection_counts[0] * inspection_counts[1]);

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
fn simulate_round(monkeys: &mut [Monkey]) -> Result<()> {
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

            // Post-inspection division by thre
            worry /= 3;

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
    fn example() {
        let mut monkeys = read_input_file("../inputs/day11_example.txt").unwrap();

        // Simulate first round and check items
        simulate_round(&mut monkeys).unwrap();
        assert_eq!(monkeys[0].items, [20, 23, 27, 26]);
        assert_eq!(monkeys[1].items, [2080, 25, 167, 207, 401, 1046]);
        assert!(monkeys[2].items.is_empty());
        assert!(monkeys[3].items.is_empty());

        // Simulate 19 more rounds and check inspection counts
        for _ in 0..19 {
            simulate_round(&mut monkeys).unwrap();
        }
        assert_eq!(monkeys[0].items_inspected_count, 101);
        assert_eq!(monkeys[1].items_inspected_count, 95);
        assert_eq!(monkeys[2].items_inspected_count, 7);
        assert_eq!(monkeys[3].items_inspected_count, 105);
    }
}
