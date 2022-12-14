use anyhow::{bail, Context, Result};
use itertools::{EitherOrBoth, Itertools};
use std::cmp::Ordering;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
enum Element {
    Number(u32),
    List(Vec<Element>)
}

type Pair = (Element, Element);

fn check_if_pair_is_in_right_order(e1: &Element, e2: &Element) -> Ordering {
    match (e1, e2) {
        (Element::Number(a), Element::Number(b)) => {
            if a < b { Ordering::Less }
            else if a == b { Ordering::Equal }
            else { Ordering::Greater }
        },
        (Element::Number(_), Element::List(_)) => {
            check_if_pair_is_in_right_order(&Element::List(vec!(e1.clone())), e2)
        },
        (Element::List(_), Element::Number(_)) => {
            check_if_pair_is_in_right_order(e1, &Element::List(vec!(e2.clone())))
        },
        (Element::List(a), Element::List(b)) => {
            for i in a.iter().zip_longest(b) {
                match i {
                    EitherOrBoth::Both(x, y) => {
                        let res = check_if_pair_is_in_right_order(x, y);
                        if res != Ordering::Equal {
                            return res;
                        }
                    },
                    EitherOrBoth::Left(_) => { return Ordering::Greater; }
                    EitherOrBoth::Right(_) => { return Ordering::Less; }
                }
            }

            return Ordering::Equal;
        }
    }
}

fn get_indices_of_correct_pairs(pairs: &[Pair]) -> Vec<usize> {
    let mut res = vec!();

    for (i, p) in pairs.iter().enumerate() {
        if check_if_pair_is_in_right_order(&p.0, &p.1) == Ordering::Less {
            res.push(i + 1);
        }
    }

    res
}

fn main() -> Result<()> {
    let pairs = read_input_file("../inputs/day13_input.txt")?;

    // First part
    println!("Sum of indices of correct pairs: {}", get_indices_of_correct_pairs(&pairs).iter().sum::<usize>());

    // Second part
    let mut all_packets: Vec<Element> = pairs.into_iter().flat_map(|p| [p.0, p.1].into_iter()).collect();
    println!("Decoder key is: {}", sort_and_get_divider_indices_product(&mut all_packets));

    Ok(())
}

fn parse_packet(expr: &str) -> Result<Element> {
    // Are we reading a list?
    let mut chars = expr.chars();
    if chars.next() == Some('[') && chars.last() == Some(']') {
        // Remove first and last character
        let mut list_str = expr.to_string();
        list_str.pop();
        list_str.remove(0);

        // Split list into elements, separating by comma (but not splitting inside sublists)
        let mut nested_level = 0;
        let mut list_elements_str = vec!();
        let mut cur_list_element = String::new();

        for c in list_str.chars() {
            if nested_level == 0 && c == ',' {
                list_elements_str.push(cur_list_element);
                cur_list_element = String::new();
            } else {
                cur_list_element.push(c);

                if c == '[' {
                    nested_level += 1;
                } else if c == ']' {
                    nested_level -= 1;
                }
            }
        }

        if nested_level != 0 {
            bail!("Nesting wrong for: {}", expr);
        }

        if !cur_list_element.is_empty() {
            list_elements_str.push(cur_list_element);
        }

        let res: Vec<Element> = list_elements_str.iter().map(|e| parse_packet(e).unwrap()).collect();  // Ugly unwrap should be handled better
        Ok(Element::List(res))
    }

    // Otherwise, the string must contain just a single number
    else if let Ok(n) = expr.parse::<u32>() {
        Ok(Element::Number(n))
    }
    
    else {
        bail!("Could not parse: {}", expr);
    }
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Vec<Pair>> {
    let input = read_to_string(input_path).context("Could not read input file!")?;
    let mut res = vec!();

    // Read input in chunks of 3 lines
    for mut chunk in &input.lines().chunks(3) {
        let pair = (
            parse_packet(chunk.next().unwrap())?,
            parse_packet(chunk.next().unwrap())?
        );

        if !chunk.next().unwrap().is_empty() {
            bail!("No empty line in between pairs found!")
        }

        res.push(pair);
    }

    Ok(res)
}

fn sort_and_get_divider_indices_product(packets: &mut Vec<Element>) -> usize {
    // Add divider packets
    let divider1 = Element::List(vec!(Element::List(vec!(Element::Number(2)))));
    let divider2 = Element::List(vec!(Element::List(vec!(Element::Number(6)))));
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    // Sort whole list of packets using comparison function
    packets.sort_unstable_by(|a, b| check_if_pair_is_in_right_order(&a, &b));

    // Determine (1-based) indices of divider packets and return product
    (packets.iter().position(|e| *e == divider1).expect("Divider packet not found") + 1) *
        (packets.iter().position(|e| *e == divider2).expect("Divider packet not found") + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let pairs = read_input_file("../inputs/day13_example.txt").unwrap();
        assert_eq!(get_indices_of_correct_pairs(&pairs).iter().sum::<usize>(), 13);

        let mut all_packets: Vec<Element> = pairs.into_iter().flat_map(|p| [p.0, p.1].into_iter()).collect();
        assert_eq!(sort_and_get_divider_indices_product(&mut all_packets), 140);
    }
}
