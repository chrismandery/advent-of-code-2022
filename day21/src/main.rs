use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug)]
enum Monkey {
    Constant { number: i64 },
    Calculation { operator: Operator, operand1: String, operand2: String } 
}

type MonkeyMap = HashMap<String, Monkey>;

/// Recursively calculates the value a monkey will shout (if ignore_humn is false for the first part of the puzzle) or the value but only
/// if the calculation tree does not contain the "humn" monkey (if ignore_humn is true for the second part of the puzzle).
fn eval_monkey(monkeys: &MonkeyMap, monkey_name: &str, ignore_humn: bool) -> Option<i64> {
    if ignore_humn && monkey_name == "humn" {
        return None;
    }

    let m = monkeys.get(monkey_name).expect("Monkey not found!");
    match m {
        Monkey::Constant { number } => { Some(*number) }
        Monkey::Calculation { operator , operand1, operand2 } => {
            if let (Some(a), Some(b)) = (eval_monkey(monkeys, operand1, ignore_humn), eval_monkey(monkeys, operand2, ignore_humn)) {
                match operator {
                    Operator::Add => Some(a + b),
                    Operator::Sub => Some(a - b),
                    Operator::Mul => Some(a * b),
                    Operator::Div => Some(a / b),
                }
            } else {
                None
            }
        }
    }
}

/// Determines the number the human needs to yell to make the equation for the root monkey valid (second part of the puzzle). Reverses the
/// calculation and determines the number that the human must yell to get the desired result. This function assumes that the subtree
/// traversed contains exactly one "humn" node, otherwise it will fail.
fn eval_human_number(monkeys: &MonkeyMap, monkey_name: &str, desired_result: i64) -> i64 {
    if monkey_name == "humn" {
        return desired_result;
    }

    let monkey = monkeys.get(monkey_name).expect("Monkey not found!");

    match monkey {
        Monkey::Calculation { operator, operand1, operand2 } => {
            let maybe_value1 = eval_monkey(monkeys, operand1, true);
            let maybe_value2 = eval_monkey(monkeys, operand2, true);

            // Exactly one of the two subtrees must yield a number and the other one must contain the human
            if maybe_value1.is_some() && maybe_value2.is_none() {
                let x = maybe_value1.unwrap();
                eval_human_number(monkeys, operand2, match operator {
                    Operator::Add => desired_result - x,
                    Operator::Sub => x - desired_result,
                    Operator::Mul => desired_result / x,
                    Operator::Div => x / desired_result
                })
            } else if maybe_value2.is_some() && maybe_value1.is_none() {
                let y = maybe_value2.unwrap();
                eval_human_number(monkeys, operand1, match operator {
                    Operator::Add => desired_result - y,
                    Operator::Sub => desired_result + y,
                    Operator::Mul => desired_result / y,
                    Operator::Div => desired_result * y
                })
            } else {
                panic!("No human found or human in both subtrees?!");
            }
        },
        _ => panic!("Monkey during eval_human_number must not contain a constant!")
    }
}

fn main() -> Result<()> {
    let mut monkeys = read_input_file("../inputs/day21_input.txt")?;

    // First part
    println!("First part - Monkey root yells: {}", eval_monkey(&monkeys, "root", false).unwrap());

    // Second part - Override operation in root monkey to subtract (this way if the difference is zero, both operands are equal)
    if let Monkey::Calculation { operator: _, operand1, operand2 } = monkeys.get("root").unwrap() {
        monkeys.insert("root".into(), Monkey::Calculation { operator: Operator::Sub, operand1: operand1.clone(), operand2: operand2.clone() });
    }
    println!("Second part - Human needs to yell: {}", eval_human_number(&monkeys, "root", 0));

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<MonkeyMap> {
    let re = Regex::new(r"^(?P<name>[a-z]{4}): ((?P<number>\d+)|(?P<operand1>[a-z]{4}) (?P<operator>[\+\-*/]) (?P<operand2>[a-z]{4}))$")?;

    let input = read_to_string(input_path).context("Could not read input file!")?;
    let res = input
        .lines()
        .map(|l| {
            let caps = re.captures(l).expect("Could not parse input line!");
            let name = caps.name("name").unwrap().as_str().into();

            if let Some(cap_number) = caps.name("number") {
                (name, Monkey::Constant { number: cap_number.as_str().parse().expect("Could not parse number!") })
            } else {
                (name, Monkey::Calculation {
                    operator: match caps.name("operator").unwrap().as_str() {
                        "+" => Operator::Add,
                        "-" => Operator::Sub,
                        "*" => Operator::Mul,
                        "/" => Operator::Div,
                        _ => panic!("Unknown operator!")
                    },
                    operand1: caps.name("operand1").unwrap().as_str().into(),
                    operand2: caps.name("operand2").unwrap().as_str().into(),
                })
            }
        })
        .collect();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let monkeys = read_input_file("../inputs/day21_example.txt").unwrap();
        assert_eq!(eval_monkey(&monkeys, "root", false).unwrap(), 152);
    }

    #[test]
    fn example_part2() {
        let mut monkeys = read_input_file("../inputs/day21_example.txt").unwrap();

        // Override operation in root monkey to subtract (this way if the difference is zero, both operands are equal)
        if let Monkey::Calculation { operator: _, operand1, operand2 } = monkeys.get("root").unwrap() {
            monkeys.insert("root".into(), Monkey::Calculation { operator: Operator::Sub, operand1: operand1.clone(), operand2: operand2.clone() });
        }

        assert_eq!(eval_human_number(&monkeys, "root", 0), 301);
    }
}
