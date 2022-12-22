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

fn eval_monkey(monkeys: &MonkeyMap, monkey_name: &str) -> i64 {
    let m = monkeys.get(monkey_name).expect("Monkey not found!");
    match m {
        Monkey::Constant { number } => { *number }
        Monkey::Calculation { operator , operand1, operand2 } => {
            let a = eval_monkey(monkeys, operand1);
            let b = eval_monkey(monkeys, operand2);

            match operator {
                Operator::Add => a + b,
                Operator::Sub => a - b,
                Operator::Mul => a * b,
                Operator::Div => a / b,
            }
        }
    }
}

fn main() -> Result<()> {
    let monkeys = read_input_file("../inputs/day21_input.txt")?;
    println!("First part - Monkey root yells: {}", eval_monkey(&monkeys, "root"));

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
    fn example() {
        let monkeys = read_input_file("../inputs/day21_example.txt").unwrap();
        assert_eq!(eval_monkey(&monkeys, "root"), 152);
    }
}
