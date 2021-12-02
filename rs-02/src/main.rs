use std::fs;
use std::io;
use std::str::FromStr;

fn main() {
    println!("part 1: {}", run(&read_input(), Position1::new()));
    println!("part 1 mini: {:?}", part_1_mini());
    println!("part 2: {}", run(&read_input(), Position2::new()));
    println!("part 2 mini: {:?}", part_2_mini());
}

/// smaller versions of the solution, based on some ideas from reddit
fn part_1_mini() -> i32 {
    include_str!("../input")
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold([0, 0], |[distance, depth], (direction, x)| {
            let x: i32 = x.parse().unwrap();
            match direction {
                "forward" => [distance + x, depth],
                "up" => [distance, depth - x],
                "down" => [distance, depth + x],
                _ => panic!("Unexpected input"),
            }
        })
        .iter()
        .product()
}

fn part_2_mini() -> i32 {
    include_str!("../input")
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold([0, 0, 0], |[distance, depth, aim], (direction, x)| {
            let x: i32 = x.parse().unwrap();
            match direction {
                "forward" => [distance + x, depth + aim * x, aim],
                "up" => [distance, depth, aim - x],
                "down" => [distance, depth, aim + x],
                _ => panic!("Unexpected input"),
            }
        })
        .iter()
        .take(2)
        .product()
}

fn read_input() -> String {
    fs::read_to_string("input").unwrap()
}

fn run(input: &str, initial_position: impl Position) -> i32 {
    input
        .lines()
        .map(|line| line.trim().parse::<Motion>().unwrap())
        .fold(initial_position, |position, motion| {
            position.update(&motion)
        })
        .calc_multiple()
}

#[derive(Debug, PartialEq)]
enum Motion {
    Up(i32),
    Down(i32),
    Forward(i32),
}
impl FromStr for Motion {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Motion, io::Error> {
        match s.split_once(" ").unwrap() {
            ("up", x) => Ok(Motion::Up(x.parse().unwrap())),
            ("down", x) => Ok(Motion::Down(x.parse().unwrap())),
            ("forward", x) => Ok(Motion::Forward(x.parse().unwrap())),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "")),
        }
    }
}

trait Position {
    fn new() -> Self;
    fn update(self: Self, motion: &Motion) -> Self;
    fn calc_multiple(self: Self) -> i32;
}

struct Position1 {
    horizontal: i32,
    depth: i32,
}
impl Position for Position1 {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }
    fn update(mut self: Self, motion: &Motion) -> Self {
        match motion {
            Motion::Forward(x) => {
                self.horizontal += x;
                self
            }
            Motion::Up(x) => {
                self.depth -= x;
                self
            }
            Motion::Down(x) => {
                self.depth += x;
                self
            }
        }
    }
    fn calc_multiple(self: Self) -> i32 {
        self.horizontal * self.depth
    }
}

struct Position2 {
    aim: i32,
    horizontal: i32,
    depth: i32,
}
impl Position for Position2 {
    fn new() -> Self {
        Self {
            aim: 0,
            horizontal: 0,
            depth: 0,
        }
    }
    fn update(mut self: Self, motion: &Motion) -> Self {
        match motion {
            Motion::Forward(x) => {
                self.horizontal += x;
                self.depth += self.aim * x;
                self
            }
            Motion::Up(x) => {
                self.aim -= x;
                self
            }
            Motion::Down(x) => {
                self.aim += x;
                self
            }
        }
    }
    fn calc_multiple(self: Self) -> i32 {
        self.horizontal * self.depth
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

    #[test]
    fn test_parse_input() {
        vec![
            ("forward 1", Motion::Forward(1)),
            ("up 2", Motion::Up(2)),
            ("down 3", Motion::Down(3)),
        ]
        .iter()
        .for_each(|(input, expected)| {
            assert_eq!(input.parse::<Motion>().unwrap(), *expected);
        });

        assert!(matches!("foobar 4".parse::<Motion>(), Err(_)));
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(run(&SAMPLE_INPUT, Position1::new()), 150);
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(run(&read_input(), Position1::new()), 2272262);
    }

    #[test]
    fn test_part_1_mini() {
        assert_eq!(part_1_mini(), 2272262);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(run(&SAMPLE_INPUT, Position2::new()), 900);
    }

    #[test]
    fn test_part_2_real() {
        assert_eq!(run(&read_input(), Position2::new()), 2134882034);
    }

    #[test]
    fn test_part_2_mini() {
        assert_eq!(part_2_mini(), 2134882034);
    }
}
