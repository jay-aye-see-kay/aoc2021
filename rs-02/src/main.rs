use std::fs;

#[derive(Debug, PartialEq)]
enum Motion {
    Up(i32),
    Down(i32),
    Forward(i32),
}

struct Position {
    aim: i32,
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn new() -> Self {
        Self {
            aim: 0,
            horizontal: 0,
            depth: 0,
        }
    }
}

fn main() {
    let parsed_input = parse_input(&read_input());
    println!("part 1: {}", part_1(&parsed_input));
    println!("part 2: {}", part_2(&parsed_input));
}

fn read_input() -> String {
    fs::read_to_string("input").unwrap()
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

fn part_1(input: &Vec<Motion>) -> i32 {
    let mut position = Position::new();
    for motion in input.iter() {
        match motion {
            Motion::Forward(v) => position.horizontal += v,
            Motion::Up(v) => position.depth -= v,
            Motion::Down(v) => position.depth += v,
        }
    }
    position.horizontal * position.depth
}

fn part_2(input: &Vec<Motion>) -> i32 {
    let mut position = Position::new();
    for motion in input.iter() {
        match motion {
            Motion::Forward(x) => {
                position.horizontal += x;
                position.depth += position.aim * x;
            }
            Motion::Up(x) => position.aim -= x,
            Motion::Down(x) => position.aim += x,
        }
    }
    position.horizontal * position.depth
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

    fn sample_parsed() -> Vec<Motion> {
        vec![
            Motion::Forward(5),
            Motion::Down(5),
            Motion::Forward(8),
            Motion::Up(3),
            Motion::Down(8),
            Motion::Forward(2),
        ]
    }

    #[test]
    fn test_parse_input() {
        let parsed_input = parse_input(&SAMPLE_INPUT);
        for (actual, expected) in parsed_input.iter().zip(sample_parsed()) {
            assert_eq!(*actual, expected);
        }
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(&sample_parsed()), 150);
    }

    #[test]
    fn test_part_1_real() {
        let parsed_input = parse_input(&read_input());
        assert_eq!(part_1(&parsed_input), 2272262);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(&sample_parsed()), 900);
    }

    #[test]
    fn test_part_2_real() {
        let parsed_input = parse_input(&read_input());
        assert_eq!(part_2(&parsed_input), 2134882034);
    }
}
