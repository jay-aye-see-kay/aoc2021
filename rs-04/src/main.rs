use std::fs;

mod Game;
mod Board;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut game = input.parse::<Game::Game>().unwrap();
    let winning_score = game.play_until_winner().unwrap();
    println!("part 1: {}", winning_score);

    let input = fs::read_to_string("input").unwrap();
    let mut game = input.parse::<Game::Game>().unwrap();
    let last_winning_score = game.play_until_last_winner().unwrap();
    println!("part 2: {}", last_winning_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample() {
        let input = fs::read_to_string("input.test").unwrap();
        let mut game = input.parse::<Game::Game>().unwrap();
        let winning_score = game.play_until_winner().unwrap();
        assert_eq!(winning_score, 4512);
    }

    #[test]
    fn test_part_1_real() {
        let input = fs::read_to_string("input").unwrap();
        let mut game = input.parse::<Game::Game>().unwrap();
        let winning_score = game.play_until_winner().unwrap();
        assert_eq!(winning_score, 54275);
    }

    #[test]
    fn test_part_2_sample() {
        let input = fs::read_to_string("input.test").unwrap();
        let mut game = input.parse::<Game::Game>().unwrap();
        let last_winning_score = game.play_until_last_winner().unwrap();
        assert_eq!(last_winning_score, 1924);
    }

    #[test]
    fn test_part_2_real() {
        let input = fs::read_to_string("input").unwrap();
        let mut game = input.parse::<Game::Game>().unwrap();
        let last_winning_score = game.play_until_last_winner().unwrap();
        assert_eq!(last_winning_score, 13158);
    }
}
