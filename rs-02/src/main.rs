use std::fs;

#[derive(Debug, PartialEq)]
enum Motion {
    Up(i32),
    Down(i32),
    Forward(i32),
}

struct Position {
    horizontal: i32,
    depth: i32,
}

fn main() {
    let input_string = fs::read_to_string("input").unwrap();
    let parsed_input = parse_input(&input_string);
    println!("part 1: {}", part_1(&parsed_input));
    println!("part 2: {}", part_2(&parsed_input));
}

fn parse_input(input: &str) -> Vec<Motion> {
    input
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.trim().split(' ').collect();
            match split_line[..] {
                ["up", number] => Motion::Up(number.parse().unwrap()),
                ["down", number] => Motion::Down(number.parse().unwrap()),
                ["forward", number] => Motion::Forward(number.parse().unwrap()),
                _ => panic!("unknown"),
            }
        })
        .collect()
}

/// count the number of measurements that are greater than the previous one
fn part_1(input: &Vec<Motion>) -> i32 {
    let mut position = Position { horizontal: 0, depth: 0 };
    for motion in input.iter() {
        match motion {
            &Motion::Forward(v) => position.horizontal += v,
            &Motion::Up(v) => position.depth -= v,
            &Motion::Down(v) => position.depth += v,
        }
    };
    position.horizontal * position.depth

}

/// count the number of measurements [summed in 3 wide windows] that are greater than the previous one
fn part_2(input: &Vec<Motion>) -> usize {
    todo!("NOT IMPLEMENTED");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";

    const SAMPLE_PARSED: [Motion; 6] = [
        Motion::Forward(5),
        Motion::Down(5),
        Motion::Forward(8),
        Motion::Up(3),
        Motion::Down(8),
        Motion::Forward(2),
    ];

    #[test]
    fn test_parse_input() {
        let sample_parsed: Vec<Motion> = SAMPLE_PARSED.into();
        for (actual, expected) in parse_input(&SAMPLE_INPUT).iter().zip(sample_parsed) {
            assert_eq!(*actual, expected);
        }
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(&SAMPLE_PARSED.into()), 150);
    }

    #[test]
    fn test_part_1_real() {
        let input_string = fs::read_to_string("input").unwrap();
        let parsed_input = parse_input(&input_string);
        assert_eq!(part_1(&parsed_input), 2272262);
    }

    // #[test]
    // fn test_part_2_sample() {
    //     assert_eq!(part_2(&SAMPLE_PARSED.to_vec()), 0);
    // }

    // #[test]
    // fn test_part_2_real() {
    //     let input_string = fs::read_to_string("input").unwrap();
    //     let parsed_input = parse_input(&input_string);
    //     assert_eq!(part_2(&parsed_input), 0);
    // }
}