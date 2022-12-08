use anyhow::{anyhow, Context, Result};
use std::fs::read_to_string;
use std::path::Path;

type ElfCalorieList = Vec<Vec<usize>>;

fn get_calorie_list_from_input<P: AsRef<Path>>(input_path: P) -> Result<ElfCalorieList> {
    let input = read_to_string(input_path)?;
    let lines = input.lines();

    let mut elves = vec!();
    let mut cur_elf_calorie_list = vec!();

    for line in lines {
        if line.is_empty() {
            elves.push(cur_elf_calorie_list);
            cur_elf_calorie_list = vec!();
        } else {
            cur_elf_calorie_list.push(line.parse().with_context(|| format!("Could not parse \"{}\" from input as a number!", line))?);
        }
    }

    elves.push(cur_elf_calorie_list);

    Ok(elves)
}

fn get_max_calories(ecl: &ElfCalorieList) -> Result<usize> {
    ecl.iter().map(|cl| cl.iter().sum()).max().ok_or(anyhow!("Elf calorie list was empty!"))
}

fn get_sum_of_top3_calories(ecl: &ElfCalorieList) -> Result<usize> {
    let mut calorie_list: Vec<usize> = ecl.iter().map(|cl| cl.iter().sum()).collect();
    calorie_list.sort_unstable();
    calorie_list.reverse();

    Ok(calorie_list[0] + calorie_list[1] + calorie_list[2])
}

fn main() -> Result<()> {
    let calorie_list_per_elf = get_calorie_list_from_input("../inputs/day1_input.txt")?;
    println!("Max calorie numbers from {} elves is: {}", calorie_list_per_elf.len(), get_max_calories(&calorie_list_per_elf)?);
    println!("Sum of three highest alorie numbers from {} elves is: {}", calorie_list_per_elf.len(), get_sum_of_top3_calories(&calorie_list_per_elf)?);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_example() {
        let calorie_list_per_elf = get_calorie_list_from_input("../inputs/day1_example.txt").unwrap();
        assert_eq!(get_max_calories(&calorie_list_per_elf).unwrap(), 24000);
        assert_eq!(get_sum_of_top3_calories(&calorie_list_per_elf).unwrap(), 45000);
    }
}
