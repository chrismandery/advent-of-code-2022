use anyhow::Result;
use std::fs::read_to_string;
use std::path::Path;

fn decimal_to_snafu(mut d: i64) -> String {
    // Determine current digit (if minus or double-minus, increase the number to compensate for it)
    let cur_digit = match d.rem_euclid(5) {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => { d += 2; '=' },
        4 => { d += 1; '-' },
        _ => panic!("Invalid remainder value- this should never happen!")
    };

    d /= 5;
    if d > 0 {
        let mut s = decimal_to_snafu(d);
        s.push(cur_digit);
        s
    } else {
        cur_digit.to_string()
    }
}

fn main() -> Result<()> {
    let snafu_numbers = read_input_file("../inputs/day25_input.txt").unwrap();
    let sum_decimal = snafu_numbers.iter().map(|s| snafu_to_decimal(&s)).sum();
    println!("Sum is: {} => as a SNAFU number: {}", sum_decimal, decimal_to_snafu(sum_decimal));

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<String>> {
    let input = read_to_string(input_path).expect("Could not read input file!");
    Ok(input.lines().map(|x| x.to_string()).collect())
}

fn snafu_to_decimal(s: &str) -> i64 {
    // Remove and parse last digit
    match s.char_indices().next_back() {
        Some((i, c)) => {
            let digit_value = match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!("Unknown digit character in SNAFU number!")
            };
            digit_value + snafu_to_decimal(&s[..i]) * 5
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let snafu_numbers = read_input_file("../inputs/day25_example.txt").unwrap();
        let decimal_numbers: Vec<i64> = snafu_numbers.iter().map(|s| snafu_to_decimal(&s)).collect();
        let mut it = decimal_numbers.iter();
        assert_eq!(it.next(), Some(&1747));
        assert_eq!(it.next(), Some(&906));
        assert_eq!(it.next(), Some(&198));
        assert_eq!(it.next(), Some(&11));
        assert_eq!(it.next(), Some(&201));
        assert_eq!(it.next(), Some(&31));
        assert_eq!(it.next(), Some(&1257));
        assert_eq!(it.next(), Some(&32));
        assert_eq!(it.next(), Some(&353));
        assert_eq!(it.next(), Some(&107));
        assert_eq!(it.next(), Some(&7));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), Some(&37));
        assert_eq!(it.next(), None);
        assert_eq!(decimal_to_snafu(4890), "2=-1=0");
    }
}
