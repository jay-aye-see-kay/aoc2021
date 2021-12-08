#![allow(unused_attributes, dead_code)]

use std::collections::HashSet;
use std::fs;

fn main() {
    println!("part 1: {}", part_1("input"));
    println!("part 2: {}", part_2("input"));
}

fn part_1(filename: &str) -> i32 {
    let mut unique_length_count = 0;
    for entry in get_input(filename).iter() {
        for digit in entry.output_digits.iter() {
            if let Some(_) = digit.value {
                unique_length_count += 1;
            }
        }
    }
    unique_length_count
}
fn part_2(filename: &str) -> i32 {
    get_input(filename)
        .iter_mut()
        .map(|entry| entry.deduce())
        .sum()
}

#[derive(Debug, Clone)]
struct Digit {
    segments: HashSet<char>,
    value: Option<usize>,
}

impl Digit {
    fn set_value(&mut self, value: usize) {
        self.value = Some(value);
    }
}

#[derive(Debug, Clone)]
struct Entry {
    all_digits: Vec<Digit>,
    output_digits: Vec<Digit>,
    is_all_known: bool,
}

impl Entry {
    fn deduce(self: &mut Self) -> i32 {
        let s4 = self
            .all_digits
            .iter()
            .find(|d| d.value == Some(4))
            .unwrap()
            .clone();
        let s7 = self
            .all_digits
            .iter()
            .find(|d| d.value == Some(7))
            .unwrap()
            .clone();

        // find 9 (4 is a subset of 9, but 0 and 6 aren't)
        let mut s9 = None;
        for digit in &mut self.all_digits.iter_mut() {
            if digit.segments.len() == 6 && digit.segments.is_superset(&s4.segments) {
                digit.set_value(9);
                s9 = Some(digit.clone());
            }
        }

        // find 0 (0 is a subset of 7, but 6 isn't)
        for digit in &mut self.all_digits.iter_mut() {
            if digit.segments.len() == 6
                && digit.segments.is_superset(&s7.segments)
                && digit.value == None
            {
                digit.set_value(0);
            }
        }

        // find 6 (it's the last remaining 6-segment digit)
        for digit in &mut self.all_digits.iter_mut() {
            if digit.segments.len() == 6 && digit.value == None {
                digit.set_value(6);
            }
        }

        // find 3 (7 is a subset of 3, but 2 and 5 aren't)
        for digit in &mut self.all_digits.iter_mut() {
            if digit.segments.len() == 5 && digit.segments.is_superset(&s7.segments) {
                digit.set_value(3);
            }
        }

        // find 5 (5 is a subset of 9, but 2 isn't)
        for digit in &mut self.all_digits.iter_mut() {
            if digit.segments.len() == 5
                && digit.segments.is_subset(&s9.as_ref().unwrap().segments)
                && digit.value == None
            {
                digit.set_value(5);
            }
        }

        // find 2 (it's the last remaining digit)
        for digit in &mut self.all_digits.iter_mut() {
            if digit.segments.len() == 5 && digit.value == None {
                digit.set_value(2);
            }
        }

        // look up self.output_digits and return a number
        let mut out = vec![];

        self.output_digits.iter().for_each(|digit| {
            self.all_digits.iter().for_each(|d| {
                if d.segments == digit.segments {
                    out.push(d.value.unwrap());
                }
            });
        });

        out.iter().rev().enumerate().fold(0, |sum, (i, x)| {
            let multiplier = 10usize.pow(i as u32);
            sum + multiplier as i32 * (*x as i32)
        })
    }
}

fn get_input(filename: &str) -> Vec<Entry> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let (all_digits_str, output_str) = line.split_once(" | ").unwrap();
            Entry {
                all_digits: parse_digit(all_digits_str),
                output_digits: parse_digit(output_str),
                is_all_known: true,
            }
        })
        .collect()
}

fn parse_digit(digit_str: &str) -> Vec<Digit> {
    digit_str
        .split_whitespace()
        .map(|digit_str| {
            let value = match digit_str.len() {
                2 => Some(1),
                4 => Some(4),
                3 => Some(7),
                7 => Some(8),
                _ => None,
            };
            Digit {
                segments: digit_str.chars().collect(),
                value,
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1("input.test"), 26);
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(part_1("input"), 355);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2("input.test"), 61229);
    }

    #[test]
    fn test_part_2_real() {
        assert_eq!(part_2("input"), 983030);
    }
}
