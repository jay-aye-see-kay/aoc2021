use std::fs;

use binary::Binary;

mod binary;

fn main() {
    println!("part 1: {}", part_1("input"));
    println!("part 2: {}", part_2("input"));
}

fn count_ones(codes: &Vec<Binary>) -> Vec<i32> {
    let binary_width = codes[0].bits.len();
    codes
        .iter()
        .fold(vec![0; binary_width], |mut counts, code| {
            for bit_idx in 0..binary_width {
                if code.value_at(bit_idx) == 1 {
                    counts[bit_idx] += 1;
                }
            }
            counts
        })
}

fn part_1(filename: &str) -> i32 {
    let codes: Vec<Binary> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let total_count = codes.len() as i32;

    let counts = count_ones(&codes);

    let gamma = Binary {
        bits: counts
            .iter()
            .map(|count| *count > total_count / 2)
            .collect(),
    };
    let epsilon = Binary {
        bits: counts
            .iter()
            .map(|count| *count < total_count / 2)
            .collect(),
    };

    gamma.to_decimal() * epsilon.to_decimal()
}

fn filter_by_position(to_keep: &bool, at_position: &usize, codes: &Vec<Binary>) -> Vec<Binary> {
    codes
        .iter()
        .filter(|code| code.bits[*at_position] == *to_keep)
        .map(|code| code.clone())
        .collect()
}

fn most_common_at(codes_count: usize, counts: &Vec<i32>, at_position: usize) -> bool {
    counts[at_position] as f64 >= codes_count as f64 / 2.0
}

fn part_2(filename: &str) -> i32 {
    let orginal_codes: Vec<Binary> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut considered_idx = 0;
    let mut codes = orginal_codes.clone();

    let oxygen_generator_rating = loop {
        let counts = count_ones(&codes);
        if codes.len() == 1 {
            break codes[0].clone();
        }
        let most_common = most_common_at(codes.len(), &counts, considered_idx);
        codes = filter_by_position(&most_common, &considered_idx, &codes);
        considered_idx += 1;
    };

    considered_idx = 0;
    codes = orginal_codes.clone();

    let co2_scrubber_rating = loop {
        let counts = count_ones(&codes);
        if codes.len() == 1 {
            break codes[0].clone();
        }
        let least_common = !most_common_at(codes.len(), &counts, considered_idx);
        codes = filter_by_position(&least_common, &considered_idx, &codes);
        considered_idx += 1;
    };

    oxygen_generator_rating.to_decimal() * co2_scrubber_rating.to_decimal()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1("input.test"), 198);
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(part_1("input"), 3912944);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2("input.test"), 230);
    }

    #[test]
    fn test_part_2_full() {
        assert_eq!(part_2("input"), 4996233);
    }
}
