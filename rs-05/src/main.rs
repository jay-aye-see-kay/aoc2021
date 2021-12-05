use std::fs;
use std::collections::HashMap;

fn main() {
    println!("part 1: {}", part_1("input"));
}

/// lay out the lines in a sparse matrix and then count the number of coordinates where two or more
/// lines overlap
fn part_1(filename: &str) -> i32 {
    let mut sparse_matrix: HashMap<Position, i32> = HashMap::new();
    for line in get_input(filename) {
        for point in line.get_points() {
            let prev_count = sparse_matrix.get(&point).unwrap_or(&0);
            let new_count = prev_count + 1;
            sparse_matrix.insert(point, new_count);
        }
    }
    let mut two_plus_count = 0;
    for (_k, v) in sparse_matrix.iter() {
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

    fn get_points(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = vec![];

        // filter out diagonals
        let has_x_change = self.start.0 != self.end.0;
        let has_y_change = self.start.1 != self.end.1;
        if has_x_change && has_y_change {
            return vec![];
        }

        // {{{ FIXME this mess
        for x in self.start.0..=self.end.0 {
            let y = self.start.1;
            positions.push((x, y));
        }
        for x in self.end.0..=self.start.0 {
            let y = self.start.1;
            positions.push((x, y));
        }
        for y in self.start.1..=self.end.1 {
            let x = self.start.0;
            positions.push((x, y));
        }
        for y in self.end.1..=self.start.1 {
            let x = self.start.0;
            positions.push((x, y));
        }
        positions.sort();
        positions.dedup();
        // }}}

        positions
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
        assert_eq!(part_1("input.test"), 5);
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(part_1("input"), 7468);
    }
}
