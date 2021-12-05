#![allow(dead_code, unused_imports)]

use std::fs;

fn main() {
    println!("part 1: {}", part_1());
}

fn part_1() -> i32 {
    // parse into bingo "boards"
    // implement a has_won method
    // implement a mark board method
    0
}

#[derive(Debug)]
struct Board {
    marks: Vec<bool>,
    values: Vec<i32>,
    width: usize,
}

impl Board {
    fn new(values: Vec<i32>, width: usize) -> Self {
        Self {
            marks: vec![false; values.len()],
            values,
            width,
        }
    }

    fn rows(&self) -> Vec<Vec<(i32, bool)>> {
        let num_rows = self.values.len() / self.width;
        let mut rows = vec![vec![]; num_rows];
        for i in 0..num_rows {
            for j in 0..self.width {
                let index = i * self.width + j;
                rows[i].push((self.values[index], self.marks[index]));
            }
        }
        rows
    }

    fn columns(&self) -> Vec<Vec<(i32, bool)>> {
        let num_cols = self.width;
        let num_rows = self.values.len() / self.width;
        let mut cols = vec![vec![]; num_cols];
        for i in 0..num_cols {
            for j in 0..num_rows {
                let index = i + j * num_cols;
                cols[i].push((self.values[index], self.marks[index]));
            }
        }
        cols
    }

    fn mark(&mut self, draw: i32) {
        for i in 0..self.values.len() {
            if self.values[i] == draw {
                self.marks[i] = true;
            }
        }
    }

    fn has_won(&self) -> bool {
        for row in self.rows() {
            if row.iter().all(|&(_, m)| m) {
                return true;
            }
        }
        for col in self.columns() {
            if col.iter().all(|&(_, m)| m) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct Game {
    boards: Vec<Board>,
    draws: Vec<i32>,
    counter: usize,
}

impl Game {
    fn new(boards: Vec<Board>, draws: Vec<i32>) -> Self {
        Self {
            boards,
            draws,
            counter: 0,
        }
    }

    fn tick(&mut self) {
        let draw = self.draws[self.counter];
        self.counter += 1;
        for board in &mut self.boards {
            board.mark(draw);
        }
    }
}

fn parse_input(input: &str) -> Game {
    let mut draws = vec![];
    let mut boards = vec![];
    for (i, input) in input.split("\n\n").enumerate() {
        if i == 0 {
            draws = input.split(",").map(|draw| draw.parse().unwrap()).collect();
        } else {
            let grid: Vec<Vec<i32>> = input
                .trim()
                .split("\n")
                .map(|line| {
                    line.trim()
                        .split_whitespace()
                        .map(|cell| cell.parse::<i32>().unwrap())
                        .collect()
                })
                .collect();
            let values = grid.iter().flatten().map(|c| *c).collect();
            let width = grid[0].len();
            boards.push(Board::new(values, width));
        }
    }
    Game::new(boards, draws)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("input.test").unwrap();
        let game = parse_input(&input);
        assert_eq!(game.boards.len(), 3);
        assert_eq!(game.boards[0].width, 5);
        assert_eq!(game.boards[0].values.len(), 25);
        assert_eq!(
            game.draws,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
    }

    #[test]
    fn test_rows() {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let board = Board::new(values, 3);
        let rows = board.rows();
        assert_eq!(rows[0], vec![(1, false), (2, false), (3, false)]);
        assert_eq!(rows[1], vec![(4, false), (5, false), (6, false)]);
        assert_eq!(rows[2], vec![(7, false), (8, false), (9, false)]);
    }

    #[test]
    fn test_columns() {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let board = Board::new(values, 3);
        let cols = board.columns();
        assert_eq!(cols[0], vec![(1, false), (4, false), (7, false)]);
        assert_eq!(cols[1], vec![(2, false), (5, false), (8, false)]);
        assert_eq!(cols[2], vec![(3, false), (6, false), (9, false)]);
    }

    #[test]
    fn test_mark() {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut board = Board::new(values, 3);
        board.mark(1);
        assert_eq!(
            board.marks,
            vec![true, false, false, false, false, false, false, false, false]
        );
    }

    #[test]
    fn test_has_won() {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut board = Board::new(values, 3);
        board.mark(1);
        board.mark(2);
        board.mark(3);
        assert!(board.has_won());
    }

    #[test]
    fn test_tick() {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let draws = vec![1, 2, 3];
        let mut game = Game::new(vec![Board::new(values, 3)], draws);
        game.tick();
        assert_eq!(
            game.boards[0].marks,
            vec![true, false, false, false, false, false, false, false, false]
        );
    }

    #[test]
    fn test_gameplay() {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let draws = vec![1, 2, 3];
        let mut game = Game::new(vec![Board::new(values, 3)], draws);

        game.tick();
        assert_eq!(game.boards[0].has_won(), false);
        assert_eq!(
            game.boards[0].marks,
            vec![true, false, false, false, false, false, false, false, false]
        );

        game.tick();
        assert_eq!(game.boards[0].has_won(), false);
        assert_eq!(
            game.boards[0].marks,
            vec![true, true, false, false, false, false, false, false, false]
        );

        game.tick();
        assert_eq!(game.boards[0].has_won(), true);
        assert_eq!(
            game.boards[0].marks,
            vec![true, true, true, false, false, false, false, false, false]
        );
    }
}
