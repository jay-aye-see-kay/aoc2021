use std::str::FromStr;

use crate::Board::*;

#[derive(Debug)]
pub struct Game {
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
        let draw = self.current_draw();
        self.counter += 1;
        for board in &mut self.boards {
            board.mark(draw);
        }
    }

    fn current_draw(&self) -> i32 {
        self.draws[self.counter]
    }

    fn last_draw(&self) -> i32 {
        self.draws[self.counter - 1]
    }

    fn winning_score(&self) -> Option<i32> {
        for board in &self.boards {
            if board.has_won() {
                return Some(board.score(self.last_draw()));
            }
        }
        None
    }

    pub fn play_until_winner(&mut self) -> Option<i32> {
        while self.counter < self.draws.len() {
            self.tick();
            if let Some(score) = self.winning_score() {
                return Some(score);
            }
        }
        None
    }

    pub fn play_until_last_winner(&mut self) -> Option<i32> {
        let num_boards = self.boards.len();
        let mut winning_boards: Vec<Option<i32>> = vec![None; num_boards];
        while self.counter < self.draws.len() {
            self.tick();
            for (board_idx, board) in self.boards.iter().enumerate() {
                if board.has_won() {
                    let score = board.score(self.last_draw());
                    winning_boards[board_idx] = Some(score);
                    if winning_boards.iter().all(|b| b.is_some()) {
                        return Some(score);
                    }
                }
            }
        }
        None
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut draws = vec![];
        let mut boards = vec![];
        for (i, input) in input.split("\n\n").enumerate() {
            if i == 0 {
                draws = input.split(',').map(|draw| draw.parse().unwrap()).collect();
            } else {
                let grid: Vec<Vec<i32>> = input
                    .trim()
                    .split('\n')
                    .map(|line| {
                        line.trim()
                            .split_whitespace()
                            .map(|cell| cell.parse::<i32>().unwrap())
                            .collect()
                    })
                    .collect();
                let values = grid.iter().flatten().copied().collect();
                let width = grid[0].len();
                boards.push(Board::new(values, width));
            }
        }
        Ok(Self::new(boards, draws))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("input.test").unwrap();
        let game = input.parse::<Game>().unwrap();
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

