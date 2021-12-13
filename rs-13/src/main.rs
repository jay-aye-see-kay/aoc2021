#![allow(unused)]

use std::fs;

fn main() {
    println!("Hello, world!");
}

type Position = (usize, usize);

#[derive(Debug, PartialEq)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug)]
struct Transparency {
    dots: Vec<Position>,
    folds: Vec<Fold>,
}

fn get_input(filename: &str) -> Transparency {
    let input = fs::read_to_string(filename).unwrap();
    let (positions_str, folds_str) = input.split_once("\n\n").unwrap();

    Transparency {
        dots: positions_str
            .lines()
            .map(|line| {
                let (x_str, y_str) = line.split_once(",").unwrap();
                (x_str.parse().unwrap(), y_str.parse().unwrap())
            })
            .collect(),
        folds: folds_str
            .lines()
            .map(|line| {
                let cleaned_line = line.replace("fold along ", "");
                let (axis, value_str) = cleaned_line.split_once("=").unwrap();
                match axis {
                    "x" => Fold::X(value_str.parse().unwrap()),
                    "y" => Fold::Y(value_str.parse().unwrap()),
                    _ => panic!("Invalid axis"),
                }
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let input = get_input("input.test");
        println!("input: {:?}", input);

        assert_eq!(input.dots.len(), 18);
        assert_eq!(input.dots[0], (6, 10));
        assert_eq!(input.dots[17], (9, 0));

        assert_eq!(input.folds.len(), 2);
        assert_eq!(input.folds[0], Fold::Y(7));
        assert_eq!(input.folds[1], Fold::X(5));
    }
}
