use std::cmp::Ordering;
use std::collections::HashMap;

// sample input: target area: x=20..30, y=-10..-5
// puzzle input: target area: x=192..251, y=-89..-59

fn main() {
    let (max_height, count) =
        simulate_many(&Area::new(192, 251, -89, -59), &(-100, -100), &(1000, 1000));
    println!("part 1: {}", max_height);
    println!("part 2: {}", count);
}

type Position = (i32, i32);
type Velocity = (i32, i32);

#[derive(Debug)]
struct Area {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Area {
    fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    /// check if the position is within the area
    fn contains(&self, position: &Position) -> bool {
        position.0 >= self.x_min
            && position.0 <= self.x_max
            && position.1 >= self.y_min
            && position.1 <= self.y_max
    }

    /// check if the position has past the area
    fn has_past(&self, position: &Position) -> bool {
        position.0 > self.x_max || position.1 < self.y_min
    }
}

fn step(position: &mut Position, velocity: &mut Velocity) {
    // The probe's x and y position increases by its x and y velocity.
    position.0 += velocity.0;
    position.1 += velocity.1;
    // Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases
    // by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it
    // is already 0.
    velocity.0 += match velocity.0.cmp(&0) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    };
    // Due to gravity, the probe's y velocity decreases by 1.
    velocity.1 -= 1;
}

/// returns None if doesn't hit, otherwise returns the maximum hight reached
fn simulate(initial_velocity: &Velocity, target_area: &Area) -> Option<i32> {
    let mut current_position = (0, 0);
    let mut current_velocity = *initial_velocity;

    let mut max_height = i32::min_value();
    while !target_area.has_past(&current_position) {
        step(&mut current_position, &mut current_velocity);
        if current_position.1 > max_height {
            max_height = current_position.1;
        }
        if target_area.contains(&current_position) {
            return Some(max_height);
        }
    }
    None
}

fn simulate_many(
    target_area: &Area,
    velocity_min: &Velocity,
    velocity_max: &Velocity,
) -> (i32, i32) {
    let mut valid_targets = HashMap::new();
    for x_velocity in velocity_min.0..=velocity_max.0 {
        for y_velocity in velocity_min.1..=velocity_max.1 {
            let velocity = (x_velocity, y_velocity);
            if let Some(max_height) = simulate(&velocity, target_area) {
                valid_targets.insert(velocity, max_height);
            }
        }
    }
    let max_height = valid_targets.values().max().unwrap();
    let count = valid_targets.len() as i32;
    (*max_height, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_contains() {
        let area = Area::new(20, 30, -10, -5);
        assert_eq!(area.contains(&(15, -3)), false);
        assert_eq!(area.contains(&(20, -10)), true);
        assert_eq!(area.contains(&(30, -5)), true);
        assert_eq!(area.contains(&(25, -4)), false);
    }

    #[test]
    fn test_area_has_past() {
        let area = Area::new(20, 30, -10, -5);
        assert_eq!(area.has_past(&(30, -10)), false);
        assert_eq!(area.has_past(&(31, -10)), true);
        assert_eq!(area.has_past(&(30, -11)), true);
    }

    #[test]
    fn test_simulate_miss() {
        assert_eq!(simulate(&(5, 2), &Area::new(20, 30, -10, -5)), None);
    }

    #[test]
    fn test_simulate_hits() {
        assert_eq!(simulate(&(7, 2), &Area::new(20, 30, -10, -5)), Some(3));
        assert_eq!(simulate(&(6, 3), &Area::new(20, 30, -10, -5)), Some(6));
        assert_eq!(simulate(&(9, 0), &Area::new(20, 30, -10, -5)), Some(0));
        assert_eq!(simulate(&(6, 9), &Area::new(20, 30, -10, -5)), Some(45));
    }

    #[test]
    fn test_simulate_pass_through() {
        assert_eq!(simulate(&(17, -4), &Area::new(20, 30, -10, -5)), None);
    }

    #[test]
    fn test_sample() {
        let (max_height, count) =
            simulate_many(&Area::new(20, 30, -10, -5), &(-10, -10), &(100, 100));
        assert_eq!(max_height, 45);
        assert_eq!(count, 112);
    }

    #[test]
    fn test_real() {
        let (max_height, count) =
            simulate_many(&Area::new(192, 251, -89, -59), &(-100, -100), &(1000, 1000));
        assert_eq!(max_height, 3916);
        assert_eq!(count, 2986);
    }
}
