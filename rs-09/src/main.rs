use std::collections::{HashMap, HashSet};
use std::{fs, usize};

type Position = (usize, usize);
type HeightMap = HashMap<Position, i32>;

fn main() {
    println!("part 1: {}", part_1("input"));
    println!("part 2: {}", part_2("input"));
}

fn part_1(filename: &str) -> i32 {
    let map = get_input(filename);
    map.iter()
        .map(|(pos, value)| {
            if is_low_point(&map, pos) {
                value + 1
            } else {
                0
            }
        })
        .sum()
}

fn part_2(filename: &str) -> usize {
    let map = get_input(filename);
    let low_points = map
        .iter()
        .filter(|(pos, _)| is_low_point(&map, pos))
        .map(|(pos, _)| pos)
        .collect::<Vec<&Position>>();

    let basins: Vec<HashSet<Position>> =
        low_points.iter().map(|pos| get_basin(&map, pos)).collect();

    let mut basin_sizes: Vec<_> = basins.iter().map(|b| b.len()).collect();
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn get_basin(map: &HeightMap, pos: &Position) -> HashSet<Position> {
    let mut basin: HashSet<Position> = HashSet::new();
    let mut unvisited: HashSet<Position> = HashSet::new();
    unvisited.insert(*pos);

    while !unvisited.is_empty() {
        let unvisited_clone = unvisited.clone();
        let next_to_visit = unvisited_clone.iter().next().unwrap();
        unvisited.remove(next_to_visit);

        let all_neighbours: HashSet<_> = get_neighbors(map, next_to_visit)
            .iter()
            .filter(|n| map.get(n).unwrap() != &9)
            .cloned()
            .collect();

        let basin_clone = basin.clone();
        let new_neighbours: HashSet<_> = all_neighbours.difference(&basin_clone).collect();
        basin.extend(new_neighbours.clone().into_iter());
        unvisited.extend(new_neighbours.clone().into_iter());
    }
    basin
}

fn is_low_point(map: &HeightMap, pos: &Position) -> bool {
    get_neighbors(map, pos)
        .iter()
        .all(|other_pos| map.get(pos).unwrap() < map.get(other_pos).unwrap())
}

fn get_neighbors(map: &HeightMap, pos: &Position) -> Vec<Position> {
    vec![
        (pos.0, pos.1.saturating_sub(1)), // above
        (pos.0.saturating_sub(1), pos.1), // left
        (pos.0, pos.1 + 1),               // below
        (pos.0 + 1, pos.1),               // right
    ]
    .iter()
    .filter(|other_pos| *other_pos != pos && map.get(other_pos).is_some())
    .cloned()
    .collect()
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
        let map = get_input("input.test");
        assert_eq!(map.get(&(0, 0)), Some(&2)); // top left
        assert_eq!(map.get(&(9, 0)), Some(&0)); // top right
        assert_eq!(map.get(&(9, 4)), Some(&8)); // bottom right
        assert_eq!(map.get(&(0, 4)), Some(&9)); // bottom left
    }

    #[test]
    fn test_get_neighbors() {
        let map = get_input("input.test");
        assert_eq!(get_neighbors(&map, &(0, 0)), vec![(0, 1), (1, 0)]);
        assert_eq!(
            get_neighbors(&map, &(1, 1)),
            vec![(1, 0), (0, 1), (1, 2), (2, 1)]
        );
        assert_eq!(get_neighbors(&map, &(9, 4)), vec![(9, 3), (8, 4)]);
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1("input.test"), 15)
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(part_1("input"), 566)
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2("input.test"), 1134)
    }

    #[test]
    fn test_part_2_real() {
        assert_eq!(part_2("input"), 891684)
    }
}
