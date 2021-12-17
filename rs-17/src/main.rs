#![allow(unused)]

fn main() {
    println!("Hello, world!");
}

type Position = (i32, i32);
type Velocity = (i32, i32);

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
        position.0 < self.x_max && position.1 < self.y_max
    }
}

fn step(position: &mut Position, velocity: &mut Velocity) {
    // The probe's x and y position increases by its x and y velocity.
    position.0 += velocity.0;
    position.1 += velocity.1;
    // Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
    if velocity.0 > 0 {
        velocity.0 -= 1
    } else if velocity.0 < 0 {
        velocity.0 += 1
    };
    // Due to gravity, the probe's y velocity decreases by 1.
    velocity.1 -= 1;
}

fn simulate(initial_velocity: &Velocity, target_area: &Area) -> bool {
    let mut current_position = (0, 0);
    let mut current_velocity = *initial_velocity;

    let mut iter_count = 0;
    while !target_area.has_past(&current_position) && iter_count < 100 {
        iter_count += 1;
        step(&mut current_position, &mut current_velocity);
        if target_area.contains(&current_position) {
            return true;
        }
    }
    false
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
    fn test_simulate_miss() {
        assert_eq!(simulate(&(5, 2), &Area::new(20, 30, -10, -5)), false);
    }

    #[test]
    fn test_simulate_hits() {
        assert_eq!(simulate(&(7, 2), &Area::new(20, 30, -10, -5)), true);
        assert_eq!(simulate(&(6, 3), &Area::new(20, 30, -10, -5)), true);
        assert_eq!(simulate(&(9, 0), &Area::new(20, 30, -10, -5)), true);
    }

    #[test]
    fn test_simulate_pass_through() {
        assert_eq!(simulate(&(17, -4), &Area::new(20, 30, -10, -5)), false);
    }
}
