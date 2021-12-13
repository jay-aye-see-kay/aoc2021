use std::fmt::{Display, Error, Formatter};
use std::fs;

fn main() {
    let input = get_input("input");
    println!("part 1: {}\n", fold_transparency(&input).dots.len());
    println!("part 2:\n{}\n\n", fold_transparency_completely(&input));
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

impl Display for Transparency {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let width = self.dots.iter().map(|(x, _)| x).max().unwrap() + 1;
        let height = self.dots.iter().map(|(_, y)| y).max().unwrap() + 1;
        let mut result = String::with_capacity(height * (width + 1));
        for y in 0..height {
            for x in 0..width {
                let is_dot = self.dots.iter().find(|dot| **dot == (x, y)).is_some();
                let char_str = if is_dot { '#' } else { '.' };
                result.push(char_str);
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
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

fn fold_transparency_completely(input: &Transparency) -> Transparency {
    let mut folded = fold_transparency(input);
    while folded.folds.len() > 0 {
        folded = fold_transparency(&folded);
    }
    folded
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

    #[test]
    fn test_part_2_sample() {
        let input = get_input("input.test");
        let folded = fold_transparency_completely(&input);
        let folded_str = format!("{}", folded);

        let expected_str = "
#####
#...#
#...#
#...#
#####
";
        let folded_str = folded_str.trim();
        let expected_str = expected_str.trim();
        println!("actual:\n{}\n", folded_str);
        println!("expected:\n{}\n", expected_str);
        assert_eq!(folded_str, expected_str);
    }
    #[test]
    fn test_part_2_real() {
        let input = get_input("input");
        let folded = fold_transparency_completely(&input);
        let folded_str = format!("{}", folded);

        let expected_str = "
..##..##...##....##.####.####.#..#.#..#
...#.#..#.#..#....#.#....#....#.#..#..#
...#.#....#..#....#.###..###..##...#..#
...#.#.##.####....#.#....#....#.#..#..#
#..#.#..#.#..#.#..#.#....#....#.#..#..#
.##...###.#..#..##..####.#....#..#..##.
";
        let folded_str = folded_str.trim();
        let expected_str = expected_str.trim();
        println!("actual:\n{}\n", folded_str);
        println!("expected:\n{}\n", expected_str);
        assert_eq!(folded_str, expected_str);
    }
}
