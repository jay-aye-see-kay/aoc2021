use std::error::Error;
use std::fs;
use std::result;
use std::str::FromStr;

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
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

fn part_1(input: &Vec<Motion>) -> i32 {
    let end_position = input
        .iter()
        .fold(Position1::new(), |position, motion| position.update(motion));

    end_position.horizontal * end_position.depth
}

fn part_2(input: &Vec<Motion>) -> i32 {
    let end_position = input
        .iter()
        .fold(Position2::new(), |position, motion| position.update(motion));
    end_position.horizontal * end_position.depth
}

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
enum Motion {
    Up(i32),
    Down(i32),
    Forward(i32),
}
impl FromStr for Motion {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Motion> {
        let split_line: Vec<&str> = s.trim().split(' ').collect();
        Ok(match split_line[..] {
            ["up", x] => Motion::Up(x.parse().unwrap()),
            ["down", x] => Motion::Down(x.parse().unwrap()),
            ["forward", x] => Motion::Forward(x.parse().unwrap()),
            _ => panic!("Unknown command"),
        })
    }
}

struct Position1 {
    horizontal: i32,
    depth: i32,
}
impl Position1 {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }
    fn move_forward(mut self: Self, x: &i32) -> Self {
        self.horizontal += x;
        self
    }
    fn move_down(mut self: Self, x: &i32) -> Self {
        self.depth += x;
        self
    }
    fn move_up(mut self: Self, x: &i32) -> Self {
        self.depth -= x;
        self
    }
    fn update(self: Self, motion: &Motion) -> Self {
        match motion {
            Motion::Forward(x) => self.move_forward(x),
            Motion::Up(x) => self.move_up(x),
            Motion::Down(x) => self.move_down(x),
        }
    }
}

struct Position2 {
    aim: i32,
    horizontal: i32,
    depth: i32,
}
impl Position2 {
    fn new() -> Self {
        Self {
            aim: 0,
            horizontal: 0,
            depth: 0,
        }
    }
    fn move_forward(mut self: Self, x: &i32) -> Self {
        self.horizontal += x;
        self.depth += self.aim * x;
        self
    }
    fn raise_aim(mut self: Self, x: &i32) -> Self {
        self.aim -= x;
        self
    }
    fn lower_aim(mut self: Self, x: &i32) -> Self {
        self.aim += x;
        self
    }
    fn update(self: Self, motion: &Motion) -> Self {
        match motion {
            Motion::Forward(x) => self.move_forward(x),
            Motion::Up(x) => self.raise_aim(x),
            Motion::Down(x) => self.lower_aim(x),
        }
    }
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
