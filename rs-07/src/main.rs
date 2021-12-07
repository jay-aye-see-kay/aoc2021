use std::fs;

fn main() {
    println!("part 1: {}", part_1("input"));
}

fn part_1(filename: &str) -> i32 {
    let input = get_input(filename);
    let median = median(&input);
    let sum: i32 = input.iter().map(|x| (x - median).abs()).sum();
    sum
}

fn median(numbers: &Vec<i32>) -> i32 {
    let mut numbers = numbers.clone();
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn get_input(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split(",")
        .map(|line| line.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let input = get_input("input.test");
        let expected = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(input, expected);
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1("input.test"), 37);
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(part_1("input"), 347011);
    }
}
