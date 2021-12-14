#![allow(unused)]

use std::fs;

#[derive(Debug, PartialEq)]
struct Rule {
    from: char,
    to: char,
    between: char,
}

type Polymer = String;

fn get_input(filename: &str) -> (Polymer, Vec<Rule>) {
    let input_str = fs::read_to_string(filename).unwrap();

    let (polymer_str, rules_str) = input_str.split_once("\n\n").unwrap();
    let rules: Vec<_> = rules_str
        .lines()
        .map(|r| {
            let (from_and_to_str, between_str) = r.split_once(" -> ").unwrap();
            Rule {
                from: from_and_to_str.chars().nth(0).unwrap(),
                to: from_and_to_str.chars().nth(1).unwrap(),
                between: between_str.chars().nth(0).unwrap(),
            }
        })
        .collect();

    (polymer_str.to_string(), rules)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let (polymer, rules) = get_input("input.test");
        assert_eq!(&polymer, "NNCB");
        assert_eq!(
            rules[0],
            Rule {
                from: 'C',
                to: 'H',
                between: 'B'
            }
        );
        assert_eq!(
            rules[15],
            Rule {
                from: 'C',
                to: 'N',
                between: 'C'
            }
        );
    }
}
