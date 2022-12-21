use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

/// Returns the name at the position given by the index (wrapping the list as needed), where the index 0 contains to the position of the
/// number zero in the list.
fn get_number_by_wrapping_index_from_zero(numbers: &[i128], index: usize) -> i128 {
    let zero_position = numbers.iter().position(|x| *x == 0).expect("No zero in list?!");
    let pos = (zero_position + index) % numbers.len();
    numbers[pos]
}

fn main() -> Result<()> {
    let mut numbers = read_input_file("../inputs/day20_input.txt")?;

    mix_sequence(&mut numbers, 1);

    println!("Part 1 - Sum of the three numbers is: {}", (
        get_number_by_wrapping_index_from_zero(&numbers, 1000) +
        get_number_by_wrapping_index_from_zero(&numbers, 2000) +
        get_number_by_wrapping_index_from_zero(&numbers, 3000)
    ));

    let numbers = read_input_file("../inputs/day20_input.txt")?;
    let mut numbers = numbers.iter().map(|x| x * 811589153).collect();

    mix_sequence(&mut numbers, 10);

    println!("Part 2 - Sum of the three numbers is: {}", (
        get_number_by_wrapping_index_from_zero(&numbers, 1000) +
        get_number_by_wrapping_index_from_zero(&numbers, 2000) +
        get_number_by_wrapping_index_from_zero(&numbers, 3000)
    ));

    Ok(())
}

fn mix_sequence(numbers: &mut Vec<i128>, num_rounds: usize) {
    let len = numbers.len();

    // Build list of increasing numbers (which is also shuffled) to determine which number to move next
    let mut order_list: Vec<usize> = (0..len).collect();

    for _ in 0..num_rounds {
        // Shuffle each number once
        for cur_order in 0..len {
            // println!("{}", numbers.iter().map(|&id| id.to_string() + " ").collect::<String>());

            // Determine where this number is now
            let old_index = order_list.iter().position(|x| *x == cur_order).expect("Order number not found?!");

            // Get number to shuffle and determine new index (wrapped)
            let cur_number = numbers[old_index];
            let new_index = (old_index as i128 + cur_number).rem_euclid(len as i128 - 1) as usize;
            // println!("\nNumber {} moves from index {} to index {}:", cur_number, old_index, new_index);

            // Move entries both in order list and in actual number list (very inefficient due to the use of Vec)
            order_list.remove(old_index);
            numbers.remove(old_index);
            order_list.insert(new_index, cur_order);
            numbers.insert(new_index, cur_number);
        }

        // println!("{}", numbers.iter().map( |&id| id.to_string() + ", ").collect::<String>());
    }
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<i128>> {
    let input = read_to_string(input_path).context("Could not read input file!")?;
    let res = input
        .lines()
        .map(|l| l.parse().expect("Could not parse number!"))
        .collect();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let mut numbers = read_input_file("../inputs/day20_example.txt").unwrap();
        mix_sequence(&mut numbers, 1);
        assert_eq!(get_number_by_wrapping_index_from_zero(&numbers, 1000), 4);
        assert_eq!(get_number_by_wrapping_index_from_zero(&numbers, 2000), -3);
        assert_eq!(get_number_by_wrapping_index_from_zero(&numbers, 3000), 2);
    }

    #[test]
    fn example_part2() {
        let numbers = read_input_file("../inputs/day20_example.txt").unwrap();
        let mut numbers = numbers.iter().map(|x| x * 811589153).collect();
        mix_sequence(&mut numbers, 10);
        assert_eq!(get_number_by_wrapping_index_from_zero(&numbers, 1000), 811589153);
        assert_eq!(get_number_by_wrapping_index_from_zero(&numbers, 2000), 2434767459);
        assert_eq!(get_number_by_wrapping_index_from_zero(&numbers, 3000), -1623178306);
    }
}
