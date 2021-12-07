use std::fs;

use binary::Binary;

mod binary;

fn main() {
    println!("part 1: {}", part_1("input"));
    println!("part 2: {}", part_2("input"));
}

fn count_ones(codes: &[Binary]) -> Vec<i32> {
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

enum Common {
    Most,
    Least,
}

fn filter_most_common_recursive(common: Common, codes: Vec<Binary>, at_position: usize) -> Binary {
    if codes.len() == 1 {
        return codes[0].clone();
    }
    let counts = count_ones(&codes);
    let most_common = (counts[at_position] as f64) >= (codes.len() as f64 / 2.0);
    let to_keep = match common {
        Common::Most => most_common,
        Common::Least => !most_common,
    };
    let filtered_codes = codes
        .iter()
        .filter(|code| code.bits[at_position] == to_keep)
        .cloned()
        .collect();
    filter_most_common_recursive(common, filtered_codes, at_position + 1)
}

fn part_2(filename: &str) -> i32 {
    let codes: Vec<Binary> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let oxygen_generator_rating = filter_most_common_recursive(Common::Most, codes.clone(), 0);
    let co2_scrubber_rating = filter_most_common_recursive(Common::Least, codes, 0);

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
