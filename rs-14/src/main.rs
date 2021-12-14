use std::collections::HashMap;
use std::fs;

fn main() {
    let (polymer, rules) = get_input("input");
    println!("part 1: {}", part_1(&polymer, &rules, 10));
}

#[derive(Debug, PartialEq)]
struct Rule {
    from: char,
    to: char,
    between: char,
}

type Polymer = Vec<char>;

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

    (polymer_str.chars().collect(), rules)
}

fn process_polymer(polymer: &Polymer, rules: &Vec<Rule>) -> Polymer {
    let mut to_insert: Vec<_> = polymer
        .windows(2)
        .map(|window| {
            let rule = rules
                .iter()
                .find(|rule| rule.from == window[0] && rule.to == window[1])
                .expect("Could not find rule");
            rule.between
        })
        .rev()
        .collect();

    polymer
        .iter()
        .flat_map(|c| match to_insert.pop() {
            Some(c2) => vec![*c, c2],
            None => vec![*c],
        })
        .collect()
}

fn part_1(polymer: &Polymer, rules: &Vec<Rule>, steps: i32) -> i32 {
    let mut polymer = polymer.clone();
    for _ in 0..steps {
        polymer = process_polymer(&polymer, rules);
    }

    let mut frequencies: HashMap<char, i32> = HashMap::new();
    for char in polymer {
        *frequencies.entry(char).or_insert(0) += 1;
    }

    let max = frequencies.iter().map(|(_, n)| n).max().unwrap();
    let min = frequencies.iter().map(|(_, n)| n).min().unwrap();

    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let (polymer, rules) = get_input("input.test");
        assert_eq!(polymer, vec!['N', 'N', 'C', 'B']);
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

    #[test]
    fn test_process_polymer() {
        let (polymer, rules) = get_input("input.test");
        let polymer = process_polymer(&polymer, &rules);
        assert_eq!(polymer, vec!['N', 'C', 'N', 'B', 'C', 'H', 'B'])
    }

    #[test]
    fn test_part_1_sample() {
        let (polymer, rules) = get_input("input.test");
        assert_eq!(part_1(&polymer, &rules, 10), 1588)
    }

    #[test]
    fn test_part_1_real() {
        let (polymer, rules) = get_input("input");
        assert_eq!(part_1(&polymer, &rules, 10), 2447)
    }
}
