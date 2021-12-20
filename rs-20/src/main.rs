use std::{collections::HashSet, fs};

fn main() {
    println!("part 1: {}", part_1("input"));
    println!("part 2: {}", part_2("input"));
}

type Position = (i32, i32);

#[derive(Debug)]
struct Image {
    enhancement_algo: Vec<bool>,
    pixels: HashSet<Position>,
    height: i32,
    width: i32,
    origin: Position,
}

#[derive(Debug)]
struct Bounds {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Image {
    fn bounds(&self) -> Bounds {
        Bounds {
            x_min: self.origin.0,
            x_max: self.origin.0 + self.width - 1,
            y_min: self.origin.1,
            y_max: self.origin.1 + self.height - 1,
        }
    }

    fn neighbors(&self, pos: &Position) -> Vec<bool> {
        [
            (-1, -1), // top left
            (0, -1),  // top
            (1, -1),  // top right
            (-1, 0),  // left
            (0, 0),   // self
            (1, 0),   // right
            (-1, 1),  // bottom left
            (0, 1),   // bottom
            (1, 1),   // bottom right
        ]
        .iter()
        .map(|diff| {
            let neighbor = (pos.0 + diff.0, pos.1 + diff.1);
            self.pixels.contains(&neighbor)
        })
        .collect()
    }

    fn enhance_pixel(&self, pixel: &Position) -> bool {
        let neighbors = self.neighbors(pixel);
        let lookup_index = bin_to_decimal(&neighbors);
        self.enhancement_algo[lookup_index]
    }

    fn enhance_image(&self) -> Self {
        let mut new_pixels = HashSet::new();
        let bounds = self.bounds();
        for x in (bounds.x_min - 100)..=(bounds.x_max + 100) {
            for y in (bounds.y_min - 100)..=(bounds.y_max + 100) {
                let new_pixel = self.enhance_pixel(&(x, y));
                if new_pixel {
                    new_pixels.insert((x, y));
                }
            }
        }
        Image {
            enhancement_algo: self.enhancement_algo.clone(),
            pixels: new_pixels,
            height: self.height + 2,
            width: self.width + 2,
            origin: (self.origin.0 - 1, self.origin.1 - 1),
        }
    }
}

fn bin_to_decimal(bits: &[bool]) -> usize {
    let mut result = 0;
    for (i, bit) in bits.iter().rev().enumerate() {
        if *bit {
            result += 1 << i;
        }
    }
    result
}

fn get_input(filename: &str) -> Image {
    let input_str = fs::read_to_string(filename).unwrap();
    let (enhancement_str, pixels_str) = input_str.split_once("\n\n").unwrap();
    let height = pixels_str.lines().count();
    let width = pixels_str.lines().nth(0).unwrap().chars().count();

    let mut pixels = HashSet::new();
    for (y, row) in pixels_str.lines().enumerate() {
        for (x, char) in row.chars().enumerate() {
            if char == '#' {
                pixels.insert((x as i32, y as i32));
            }
        }
    }

    Image {
        enhancement_algo: enhancement_str
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => panic!("expected '.' or '#' got '{}'", c),
            })
            .collect(),
        pixels,
        height: height as i32,
        width: width as i32,
        origin: (0, 0),
    }
}

fn part_1(filename: &str) -> usize {
    let mut image = get_input(filename);
    for _ in 0..2 {
        image = image.enhance_image();
    }
    let bounds = image.bounds();
    image
        .pixels
        .into_iter()
        .filter(|(x, y)| {
            x >= &bounds.x_min && x <= &bounds.x_max && y >= &bounds.y_min && y <= &bounds.y_max
        })
        .count()
}

fn part_2(filename: &str) -> usize {
    let mut image = get_input(filename);
    for _ in 0..50 {
        image = image.enhance_image();
    }
    let bounds = image.bounds();
    image
        .pixels
        .into_iter()
        .filter(|(x, y)| {
            x >= &bounds.x_min && x <= &bounds.x_max && y >= &bounds.y_min && y <= &bounds.y_max
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let image = get_input("input.test");
        assert_eq!(image.pixels.contains(&(0, 0)), true);
        assert_eq!(image.pixels.contains(&(0, 1)), true);
        assert_eq!(image.pixels.contains(&(1, 0)), false);
        assert_eq!(image.pixels.contains(&(4, 4)), true);
        assert_eq!(image.enhancement_algo.len(), 512);
    }

    #[test]
    fn test_min_max() {
        let image = get_input("input.test");
        let bounds = image.bounds();
        assert_eq!(bounds.x_min, 0);
        assert_eq!(bounds.x_max, 4);
        assert_eq!(bounds.y_min, 0);
        assert_eq!(bounds.y_max, 4);
    }

    #[test]
    fn test_neighbors() {
        let image = get_input("input.test");

        // check center position (example from problem description)
        let neighbors = image.neighbors(&(2, 2));
        let expected = vec![
            false, false, false, //
            true, false, false, //
            false, true, false, //
        ];
        assert_eq!(neighbors, expected);

        // check (0, 0) so we know it handles negative numbers
        let neighbors = image.neighbors(&(0, 0));
        let expected = vec![
            false, false, false, //
            false, true, false, //
            false, true, false, //
        ];
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn test_enhance_pixel() {
        let image = get_input("input.test");
        let new_pixel = image.enhance_pixel(&(2, 2));
        assert_eq!(new_pixel, true);
        let new_pixel = image.enhance_pixel(&(0, 0));
        assert_eq!(new_pixel, false);
    }

    #[test]
    fn test_enhance_image() {
        let image = get_input("input.test");
        assert_eq!(image.pixels.len(), 10);
        let image = image.enhance_image();
        assert_eq!(image.pixels.len(), 24);
        let image = image.enhance_image();
        assert_eq!(image.pixels.len(), 35);
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1("input.test"), 35);
    }

    #[test]
    #[ignore] // disabled because very slow
    fn test_part_1_real() {
        assert_eq!(part_1("input"), 4928);
    }

    #[test]
    #[ignore] // disabled because very slow
    fn test_part_2_sample() {
        assert_eq!(part_2("input.test"), 3351);
    }

    #[test]
    #[ignore] // disabled because very slow
    fn test_part_2_real() {
        assert_eq!(part_2("input"), 16605);
    }
}
