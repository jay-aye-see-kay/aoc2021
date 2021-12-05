#[derive(Debug)]
pub struct Board {
    pub marks: Vec<bool>,
    pub values: Vec<i32>,
    pub width: usize,
}

impl Board {
    pub fn new(values: Vec<i32>, width: usize) -> Self {
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

    pub fn mark(&mut self, draw: i32) {
        for i in 0..self.values.len() {
            if self.values[i] == draw {
                self.marks[i] = true;
            }
        }
    }

    pub fn has_won(&self) -> bool {
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

    fn sum_of_unmarked(&self) -> i32 {
        self.values
            .iter()
            .zip(self.marks.iter())
            .filter(|&(_, m)| !m)
            .map(|(v, _)| v)
            .sum::<i32>()
    }

    pub fn score(&self, last_draw: i32) -> i32 {
        self.sum_of_unmarked() * last_draw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
