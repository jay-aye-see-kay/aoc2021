use std::fs;

fn main() {
    println!("part 1: {}", part_1("input"));
}

fn part_1(filename: &str) -> i32 {
    let mut unique_length_count = 0;

    let input = fs::read_to_string(filename).unwrap();
    for line in input.lines() {
        let (_, output_str) = line.split_once(" | ").unwrap();
        let output: Vec<&str> = output_str.split_whitespace().map(|s| s).collect();
        for item in output {
            match item.len() {
                2 => unique_length_count += 1, // 1
                4 => unique_length_count += 1, // 4
                3 => unique_length_count += 1, // 7
                7 => unique_length_count += 1, // 8
                _ => {}
            }
        }
    }

    unique_length_count
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
        assert_eq!(part_1("input"), 26);
    }
}
