use std::collections::HashMap;
use std::fs;

type Position = (usize, usize);
type HeightMap = HashMap<Position, i32>;

fn main() {
    println!("part 1: {}", part_1("input"));
}

fn part_1(filename: &str) -> i32 {
    let input = get_input(filename);
    input
        .iter()
        .map(|(pos, value)| {
            if is_low_point(&input, pos) {
                value + 1
            } else {
                0
            }
        })
        .sum()
}

fn is_lower_than(map: &HeightMap, pos: &Position, other_pos: &Position) -> bool {
    let value = map.get(pos).unwrap();
    let other_value = map.get(other_pos);
    if pos == other_pos {
        // saturating_sub was performed, ignore this check
        return true;
    }
    match other_value {
        Some(other_val) => value < other_val,
        None => true,
    }
}

fn is_low_point(map: &HeightMap, pos: &Position) -> bool {
    is_lower_than(map, pos, &(pos.0, pos.1.saturating_sub(1))) && // above
    is_lower_than(map, pos, &(pos.0.saturating_sub(1), pos.1)) && // left
    is_lower_than(map, pos, &(pos.0, pos.1 + 1)) && // below
    is_lower_than(map, pos, &(pos.0 + 1, pos.1)) // right
}

fn get_input(filename: &str) -> HeightMap {
    let input = fs::read_to_string(filename).unwrap();
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert((x, y), char.to_digit(10).unwrap() as i32);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let input = get_input("input.test");
        assert_eq!(input.get(&(0, 0)), Some(&2)); // top left
        assert_eq!(input.get(&(9, 0)), Some(&0)); // top right
        assert_eq!(input.get(&(9, 4)), Some(&8)); // bottom right
        assert_eq!(input.get(&(0, 4)), Some(&9)); // bottom left
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1("input.test"), 15)
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(part_1("input"), 566)
    }
}
