use std::fs;

fn main() {
    let input_string = fs::read_to_string("input").unwrap();
    let parsed_input = parse_input(&input_string);
    println!("part 1: {}", part_1(&parsed_input));
    println!("part 2: {}", part_2(&parsed_input));
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

/// count the number of measurements that are greater than the previous one
fn part_1(measurements: &Vec<i32>) -> usize {
    measurements
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

/// count the number of measurements [summed in 3 wide windows] that are greater than the previous one
fn part_2(measurements: &Vec<i32>) -> usize {
    let summed_measurements: Vec<i32> = measurements
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();

    part_1(&summed_measurements)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    const SMALL_PARSED: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(&SMALL_INPUT), SMALL_PARSED);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&SMALL_PARSED.to_vec()), 7);
    }

    #[test]
    fn test_part_1_real() {
        let input_string = fs::read_to_string("input").unwrap();
        let parsed_input = parse_input(&input_string);
        assert_eq!(part_1(&parsed_input), 1462);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&SMALL_PARSED.to_vec()), 5);
    }

    #[test]
    fn test_part_2_real() {
        let input_string = fs::read_to_string("input").unwrap();
        let parsed_input = parse_input(&input_string);
        assert_eq!(part_2(&parsed_input), 1497);
    }
}
