#![allow(unused)]

use std::fmt::{Display, Error, Formatter};
use std::fs;

fn main() {
    println!("Hello, world!");
}

type Position = (usize, usize);

#[derive(Debug, PartialEq, Clone)]
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

fn fold_transparency(initial_transparency: &Transparency) -> Transparency {
    let mut new_folds = initial_transparency.folds.clone();
    let active_fold = new_folds.remove(0);

    let mut new_dots: Vec<_> = initial_transparency
        .dots
        .iter()
        .map(|(x, y)| match active_fold {
            Fold::X(axis) => {
                if *x > axis {
                    let distance_past_axis = x - axis;
                    (axis - distance_past_axis, *y)
                } else {
                    (*x, *y)
                }
            }
            Fold::Y(axis) => {
                if *y > axis {
                    let distance_past_axis = y - axis;
                    (*x, axis - distance_past_axis)
                } else {
                    (*x, *y)
                }
            }
        })
        .collect();
    new_dots.sort_unstable();
    new_dots.dedup();

    Transparency {
        folds: new_folds,
        dots: new_dots,
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

    #[test]
    fn test_part_1_sample() {
        let input = get_input("input.test");
        let folded = fold_transparency(&input);
        assert_eq!(folded.dots.len(), 17);
    }

    #[test]
    fn test_part_1_real() {
        let input = get_input("input");
        let folded = fold_transparency(&input);
        assert_eq!(folded.dots.len(), 592);
    }
}
