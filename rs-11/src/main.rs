#![allow(unused)]

use std::fmt::{Display, Error, Formatter};
use std::{collections::HashMap, fs};

fn main() {
    let mut octo_grid = get_input("input.test");
    println!("part 1: {}", octo_grid.run_and_count(100));
    println!("part 2: {}", octo_grid.run_until_all_flash());
}

type Position = (usize, usize);

#[derive(Debug, Clone)]
struct OctoGrid {
    grid: HashMap<Position, u32>,
    width: usize,
    height: usize,
    tick_count: usize,
}

impl Display for OctoGrid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut result = String::with_capacity(self.height * self.width);
        for y in 0..self.width {
            for x in 0..self.height {
                let char_int = self.grid.get(&(x, y)).unwrap();
                let char_str = match char::from_digit(*char_int, 10) {
                    Some(char_str) => char_str,
                    None => 'x', // happens with number > 9
                };
                result.push(char_str);
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

impl OctoGrid {
    fn get_neighbors(&self, pos: &Position) -> Vec<Position> {
        let mut res: Vec<_> = vec![
            (pos.0, pos.1.saturating_sub(1)),                   // above
            (pos.0.saturating_sub(1), pos.1),                   // left
            (pos.0, pos.1 + 1),                                 // below
            (pos.0 + 1, pos.1),                                 // right
            (pos.0.saturating_sub(1), pos.1.saturating_sub(1)), // above left
            (pos.0 + 1, pos.1.saturating_sub(1)),               // above right
            (pos.0.saturating_sub(1), pos.1 + 1),               // below left
            (pos.0 + 1, pos.1 + 1),                             // below right
        ]
        .iter()
        .filter(|other_pos| *other_pos != pos && self.grid.get(other_pos).is_some())
        .cloned()
        .collect();
        res.sort();
        res.dedup();
        res
    }

    fn flash_one(&mut self, pos: &Position) {
        self.get_neighbors(pos).iter_mut().for_each(|p| {
            *self.grid.get_mut(p).unwrap() += 1;
        });
    }

    fn tick(&mut self) -> i32 {
        // increment all
        for value in self.grid.values_mut() {
            *value += 1;
        }

        let mut flashed_this_tick: Vec<Position> = vec![];
        loop {
            // get list of octopuses to flash
            let will_flash: Vec<_> = self
                .grid
                .iter()
                .filter(|(_, val)| **val > 9)
                .filter(|(pos, _)| !flashed_this_tick.contains(pos))
                .map(|(pos, _)| pos)
                .cloned()
                .collect();

            // flash them
            will_flash.iter().for_each(|pos| self.flash_one(pos));
            flashed_this_tick.extend(will_flash.iter());

            // exit condition
            if will_flash.len() == 0 {
                break;
            }
        }

        // set those over 9 to 0
        self.grid.iter_mut().for_each(|(pos, val)| {
            if *val > 9 {
                *val = 0;
            }
        });

        self.tick_count += 1;
        flashed_this_tick.len() as i32
    }

    fn run_and_count(&mut self, count: i32) -> i32 {
        (0..count).map(|_| self.tick()).sum()
    }

    fn run_until_all_flash(&mut self) -> i32 {
        loop {
            let flash_count = self.tick() as usize;
            if flash_count == self.grid.iter().len() {
                break;
            }
        }
        self.tick_count as i32
    }
}

fn get_input(filename: &str) -> OctoGrid {
    let mut octo = OctoGrid {
        grid: HashMap::new(),
        width: 0,
        height: 0,
        tick_count: 0,
    };
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                octo.width = x + 1;
                octo.height = y + 1;
                octo.grid.insert((x, y), char.to_digit(10).unwrap());
            })
        });
    octo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let input_str = fs::read_to_string("input.test").unwrap();
        let octo_grid = get_input("input.test");
        let octo_grid_str = format!("{}", octo_grid);
        assert_eq!(octo_grid_str, input_str);
    }

    #[test]
    fn test_increment() {
        let expected_str = fs::read_to_string("input.test.step1").unwrap();
        let mut octo_grid = get_input("input.test");
        octo_grid.tick();
        let octo_grid_str = format!("{}", octo_grid);
        assert_eq!(octo_grid_str, expected_str);
    }

    #[test]
    fn test_increment_and_flash() {
        let expected_str = fs::read_to_string("input.test.step2").unwrap();
        let mut octo_grid = get_input("input.test.step1");
        let flash_count = octo_grid.tick();
        let octo_grid_str = format!("{}", octo_grid);
        assert_eq!(octo_grid_str, expected_str);
        assert_eq!(flash_count, 35);
    }

    #[test]
    fn test_part_1_sample() {
        let mut octo_grid = get_input("input.test");
        let mut flash_count = octo_grid.run_and_count(100);
        assert_eq!(flash_count, 1656);
    }

    #[test]
    fn test_part_1_real() {
        let mut octo_grid = get_input("input");
        let mut flash_count = octo_grid.run_and_count(100);
        assert_eq!(flash_count, 1620);
    }

    #[test]
    fn test_part_2_sample() {
        let mut octo_grid = get_input("input.test");
        let mut first_all_flash = octo_grid.run_until_all_flash();
        assert_eq!(first_all_flash, 195);
    }

    #[test]
    fn test_part_2_real() {
        let mut octo_grid = get_input("input");
        let mut first_all_flash = octo_grid.run_until_all_flash();
        assert_eq!(first_all_flash, 371);
    }
}
