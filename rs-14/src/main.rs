use std::collections::HashMap;
use std::fs;

fn main() {
    let (polymer, rules) = get_input("input");
    println!("part 1: {}", part_1(&polymer, &rules, 10));
}

// (from, to), between
type Rules = HashMap<(char, char), char>;

type Polymer = Vec<char>;

fn get_input(filename: &str) -> (Polymer, Rules) {
    let input_str = fs::read_to_string(filename).unwrap();

    let (polymer_str, rules_str) = input_str.split_once("\n\n").unwrap();
    let rules = rules_str
        .lines()
        .map(|r| {
            let (from_and_to_str, between_str) = r.split_once(" -> ").unwrap();
            let from = from_and_to_str.chars().nth(0).unwrap();
            let to = from_and_to_str.chars().nth(1).unwrap();
            let between = between_str.chars().nth(0).unwrap();
            ((from, to), between)
        })
        .collect();

    (polymer_str.chars().collect(), rules)
}

fn process_polymer(polymer: &mut Polymer, rules: &Rules) -> Polymer {
    for i in (1..polymer.len()).rev() {
        println!("i: {:?}", i);
        let between = rules.get(&(polymer[i - 1], polymer[i])).unwrap();
        polymer.insert(i, *between)
    }
    polymer.to_vec()
}

fn part_1(polymer: &Polymer, rules: &Rules, steps: i64) -> i64 {
    let mut polymer = polymer.clone();
    for _ in 0..steps {
        polymer = process_polymer(&mut polymer, rules);
    }

    let mut frequencies: HashMap<char, i64> = HashMap::new();
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
        assert_eq!(rules.get(&('C', 'H')).unwrap(), &'B');
        assert_eq!(rules.get(&('C', 'N')).unwrap(), &'C');
    }

    #[test]
    fn test_process_polymer() {
        let (mut polymer, rules) = get_input("input.test");
        let polymer = process_polymer(&mut polymer, &rules);
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

    #[test]
    #[ignore]
    fn test_part_2_sample() {
        let (polymer, rules) = get_input("input.test");
        assert_eq!(part_1(&polymer, &rules, 20), 2188189693529)
    }
}
