use std::collections::HashMap;
use std::fs;

fn main() {
    let (polymer, rules) = get_input("input");
    println!("part 1: {}", grow_polymer(&polymer, &rules, 10));
    println!("part 2: {}", grow_polymer(&polymer, &rules, 40));
}

#[derive(Clone)]
struct Polymer {
    pairs: HashMap<String, i64>,
    first_char: String,
}

type Rules = HashMap<String, (String, String)>;

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
            let creates = (
                from.to_string() + &between.to_string(),
                between.to_string() + &to.to_string(),
            );
            (from_and_to_str.to_string(), creates)
        })
        .collect();

    let polymer_pairs: Vec<String> = polymer_str
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|window| window[0].to_string() + &window[1].to_string())
        .collect();

    let mut polymer_pairs_batched: HashMap<String, i64> = HashMap::new();
    for pair in polymer_pairs {
        *polymer_pairs_batched.entry(pair).or_insert(0) += 1;
    }

    (
        Polymer {
            pairs: polymer_pairs_batched,
            first_char: polymer_str.chars().nth(0).unwrap().to_string(),
        },
        rules,
    )
}

fn increment_polymer(polymer: &Polymer, rules: &Rules) -> Polymer {
    let mut new_polymer_pairs: HashMap<String, i64> = HashMap::new();
    for (pair, count) in &polymer.pairs {
        let (new_pair_1, new_pair_2) = rules.get(pair).unwrap();
        *new_polymer_pairs.entry(new_pair_1.to_string()).or_insert(0) += count;
        *new_polymer_pairs.entry(new_pair_2.to_string()).or_insert(0) += count;
    }
    Polymer {
        pairs: new_polymer_pairs,
        first_char: polymer.first_char.to_string(),
    }
}

fn grow_polymer(polymer: &Polymer, rules: &Rules, steps: i64) -> i64 {
    let mut polymer: Polymer = polymer.clone();
    for _ in 0..steps {
        polymer = increment_polymer(&polymer, rules);
    }
    let mut sum_of_second_values: HashMap<String, i64> = HashMap::new();
    for (k, v) in polymer.pairs {
        let second_char = k.chars().nth(1).unwrap().to_string();
        *sum_of_second_values.entry(second_char).or_insert(0) += v;
    }
    *sum_of_second_values.entry(polymer.first_char).or_insert(0) += 1;

    let max = sum_of_second_values.values().max().unwrap();
    let min = sum_of_second_values.values().min().unwrap();
    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let (polymer, rules) = get_input("input.test");
        assert_eq!(polymer.pairs.get("NN").unwrap(), &1);
        assert_eq!(polymer.pairs.get("NC").unwrap(), &1);
        assert_eq!(polymer.pairs.get("CB").unwrap(), &1);
        assert_eq!(
            rules.get("CH").unwrap(),
            &("CB".to_string(), "BH".to_string())
        );
        assert_eq!(
            rules.get("CN").unwrap(),
            &("CC".to_string(), "CN".to_string())
        );
    }

    #[test]
    fn test_increment_polymer() {
        let (polymer, rules) = get_input("input.test");
        let polymer = increment_polymer(&polymer, &rules);
        assert_eq!(polymer.pairs.get("BC").unwrap(), &1);
        assert_eq!(polymer.pairs.get("CH").unwrap(), &1);
        assert_eq!(polymer.pairs.get("CN").unwrap(), &1);
        assert_eq!(polymer.pairs.get("HB").unwrap(), &1);
        assert_eq!(polymer.pairs.get("NB").unwrap(), &1);
        assert_eq!(polymer.pairs.get("NC").unwrap(), &1);
        let polymer = increment_polymer(&polymer, &rules);
        assert_eq!(polymer.pairs.get("BB").unwrap(), &2);
    }

    #[test]
    fn test_part_1_sample() {
        let (polymer, rules) = get_input("input.test");
        assert_eq!(grow_polymer(&polymer, &rules, 10), 1588)
    }

    #[test]
    fn test_part_1_real() {
        let (polymer, rules) = get_input("input");
        assert_eq!(grow_polymer(&polymer, &rules, 10), 2447)
    }

    #[test]
    fn test_part_2_sample() {
        let (polymer, rules) = get_input("input.test");
        assert_eq!(grow_polymer(&polymer, &rules, 40), 2188189693529)
    }

    #[test]
    fn test_part_2_real() {
        let (polymer, rules) = get_input("input");
        assert_eq!(grow_polymer(&polymer, &rules, 40), 3018019237563)
    }
}
