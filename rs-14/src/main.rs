use std::collections::HashMap;
use std::fs;

// split into pairs each pair creates two new pairs each tick
// set up a table of input -> creates ie "AB" -> ("AC", CB")

fn main() {
    let (polymer, rules) = get_input("input");
    println!("part 1: {}", part_1(&polymer, &rules, 14));
    println!("part 2: {}", part_2(&polymer, &rules));
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

fn process_polymer(polymer: &Polymer, rules: &Rules) -> Polymer {
    polymer
        .windows(2)
        .enumerate()
        .flat_map(|(i, window)| {
            let between = rules.get(&(window[0], window[1])).unwrap();
            if i == 0 {
                vec![window[0], *between, window[1]]
            } else {
                vec![*between, window[1]]
            }
        })
        .collect()
}

fn process_stream<'a>(
    polymer: impl Iterator<Item = &'a char>,
    rules: &'a Rules,
) -> impl Iterator<Item = &'a char> {
    // fn process_stream<'a>(polymer: Iter<'a, char>, rules: &Rules) -> Iter<'a, char> {
    let mut prev: Option<&char> = None;

    polymer.flat_map(move |char| {
        let between = match prev {
            Some(prev) => Some(rules.get(&(*prev, *char)).unwrap()),
            None => None,
        };
        prev = Some(char);
        match between {
            Some(between) => vec![between, char],
            None => vec![char],
        }
    })
}

fn get_min_max(polymer: &Polymer) -> (i64, i64) {
    let mut frequencies: HashMap<char, i64> = HashMap::new();
    for char in polymer {
        *frequencies.entry(*char).or_insert(0) += 1;
    }

    println!("frequencies: {:?}", frequencies);
    let max = frequencies.iter().map(|(_, n)| n).max().unwrap();
    let min = frequencies.iter().map(|(_, n)| n).min().unwrap();

    (*min, *max)
}

fn get_min_max_stream<'a>(polymer: impl Iterator<Item = &'a char>) -> (i64, i64) {
    let mut frequencies: HashMap<char, i64> = HashMap::new();
    for char in polymer {
        *frequencies.entry(*char).or_insert(0) += 1;
    }

    println!("frequencies: {:?}", frequencies);
    let max = frequencies.iter().map(|(_, n)| n).max().unwrap();
    let min = frequencies.iter().map(|(_, n)| n).min().unwrap();

    (*min, *max)
}

fn part_2(polymer: &Polymer, rules: &Rules) -> i64 {
    let s1 = process_stream(polymer.iter(), rules);
    let s2 = process_stream(s1, rules);
    let s3 = process_stream(s2, rules);
    let s4 = process_stream(s3, rules);
    let s5 = process_stream(s4, rules);
    let s6 = process_stream(s5, rules);
    let s7 = process_stream(s6, rules);
    let s8 = process_stream(s7, rules);
    let s9 = process_stream(s8, rules);
    let s10 = process_stream(s9, rules);
    // let s11 = process_stream(s10, rules);
    // let s12 = process_stream(s11, rules);
    // let s13 = process_stream(s12, rules);
    // let s14 = process_stream(s13, rules);
    // let s15 = process_stream(s14, rules);
    // let s16 = process_stream(s15, rules);
    // let s17 = process_stream(s16, rules);
    // let s18 = process_stream(s17, rules);
    // let s19 = process_stream(s18, rules);
    // let s20 = process_stream(s19, rules);

    let (min, max) = get_min_max_stream(s10);
    max - min
}

fn part_1(polymer: &Polymer, rules: &Rules, steps: i64) -> i64 {
    let mut polymer = polymer.clone();
    for _ in 0..steps {
        polymer = process_polymer(&polymer, rules);
    }
    let (min, max) = get_min_max(&polymer);
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
        let (polymer, rules) = get_input("input.test");
        let polymer = process_polymer(&polymer, &rules);
        assert_eq!(polymer, vec!['N', 'C', 'N', 'B', 'C', 'H', 'B'])
    }

    #[test]
    fn test_process_polymer_stream() {
        let (polymer, rules) = get_input("input.test");
        let polymer = process_stream(polymer.iter(), &rules);
        assert_eq!(
            polymer.collect::<Vec<_>>(),
            vec![&'N', &'C', &'N', &'B', &'C', &'H', &'B']
        )
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
    fn test_part_2_stream() {
        let (polymer, rules) = get_input("input.test");
        assert_eq!(part_2(&polymer, &rules), 1588)
    }

    #[test]
    // #[ignore]
    fn test_part_2_sample() {
        let (mut polymer, rules) = get_input("input.test");
        for _ in 0..7 {
            polymer = process_polymer(&polymer, &rules);
            println!(
                "polymer: {:?}",
                polymer
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            );
        }
        get_min_max(&polymer);

        assert!(false);
        // assert_eq!(part_1(&polymer, &rules, 20), 2188189693529)
    }
}
