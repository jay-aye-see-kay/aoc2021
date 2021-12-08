use std::collections::HashMap;
use std::{cmp, fs};

fn main() {
    println!("part 1: {}", count_overlapping_lines("input", false));
    println!("part 2: {}", count_overlapping_lines("input", true));
}

/// lay out the lines in a sparse matrix and then count the number of coordinates where two or more
/// lines overlap
fn count_overlapping_lines(filename: &str, consider_diagonals: bool) -> i32 {
    let mut points_sparse_matrix: HashMap<Position, i32> = HashMap::new();
    for line in get_input(filename) {
        for point in line.get_points(consider_diagonals) {
            let prev_count = points_sparse_matrix.get(&point).unwrap_or(&0);
            let new_count = prev_count + 1;
            points_sparse_matrix.insert(point, new_count);
        }
    }
    let mut two_plus_count = 0;
    for (_k, v) in points_sparse_matrix.iter() {
        if v >= &2 {
            two_plus_count += 1;
        }
    }
    two_plus_count
}

type Position = (i32, i32);

#[derive(Debug, PartialEq, Eq)]
struct Line {
    start: Position,
    end: Position,
}

impl Line {
    fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    fn get_points(&self, consider_diagonals: bool) -> Vec<Position> {
        let has_x_change = self.start.0 != self.end.0;
        let has_y_change = self.start.1 != self.end.1;

        match (has_x_change, has_y_change) {
            (true, false) => {
                // line is horizonal
                let start_x = cmp::min(self.start.0, self.end.0);
                let end_x = cmp::max(self.start.0, self.end.0);
                let y = self.start.1;
                (start_x..=end_x).map(|x| (x, y)).collect()
            }
            (false, true) => {
                // line is vertical
                let start_y = cmp::min(self.start.1, self.end.1);
                let end_y = cmp::max(self.start.1, self.end.1);
                let x = self.start.0;
                (start_y..=end_y).map(|y| (x, y)).collect()
            }
            (true, true) => {
                // line is diagonal
                if consider_diagonals {
                    self.get_diagonal_points()
                } else {
                    vec![]
                }
            }
            _ => panic!("unexpected line with no direction"),
        }
    }

    fn get_diagonal_points(&self) -> Vec<Position> {
        let mut as_array = [self.start, self.end];
        as_array.sort_unstable();
        let [start_pos, end_pos] = as_array;

        (start_pos.0..=end_pos.0)
            .enumerate()
            .map(|(i, x)| {
                let y = if start_pos.1 < end_pos.1 {
                    start_pos.1 + i as i32
                } else {
                    start_pos.1 - i as i32
                };
                (x, y)
            })
            .collect()
    }
}

fn get_input(filename: &str) -> Vec<Line> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let (start_str, end_str) = line.split_once(" -> ").unwrap();
            let start = start_str.split_once(",").unwrap();
            let end = end_str.split_once(",").unwrap();
            Line::new(
                (start.0.parse().unwrap(), start.1.parse().unwrap()),
                (end.0.parse().unwrap(), end.1.parse().unwrap()),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        assert_eq!(
            get_input("input.test"),
            vec![
                Line::new((0, 9), (5, 9)),
                Line::new((8, 0), (0, 8)),
                Line::new((9, 4), (3, 4)),
                Line::new((2, 2), (2, 1)),
                Line::new((7, 0), (7, 4)),
                Line::new((6, 4), (2, 0)),
                Line::new((0, 9), (2, 9)),
                Line::new((3, 4), (1, 4)),
                Line::new((0, 0), (8, 8)),
                Line::new((5, 5), (8, 2)),
            ]
        );
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(count_overlapping_lines("input.test", false), 5);
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(count_overlapping_lines("input", false), 7468);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(count_overlapping_lines("input.test", true), 12);
    }

    #[test]
    fn test_part_2_real() {
        assert_eq!(count_overlapping_lines("input", true), 22364);
    }
}
