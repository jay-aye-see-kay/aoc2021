use std::fs;

fn main() {
    println!("part 1: {}", find_school_size_at_time("input", 80));
}

#[derive(Debug)]
struct Fish {
    age: i32,
}

impl Fish {
    fn new(age: i32) -> Self {
        Self { age }
    }

    fn age(&mut self) -> bool {
        self.age -= 1;
        if self.age < 0 {
            self.age = 6;
            // returning true means the fish has given birth
            true
        } else {
            false
        }
    }
}

struct School {
    fish: Vec<Fish>,
}

impl School {
    fn new(fish: Vec<i32>) -> Self {
        Self {
            fish: fish.into_iter().map(|age| Fish::new(age)).collect(),
        }
    }

    fn age(&mut self) {
        let mut new_fish_count = 0;
        for fish in &mut self.fish {
            let new_fish = fish.age();
            if new_fish {
                new_fish_count += 1;
            }
        }
        for _ in 0..new_fish_count {
            self.fish.push(Fish::new(8));
        }
    }
}

fn find_school_size_at_time(filename: &str, time: i32) -> usize {
    let mut school = School::new(parse_input(filename));
    for _ in 0..time {
        school.age();
    }
    school.fish.len()
}

fn parse_input(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .expect("file not found")
        .trim()
        .split(",")
        .map(|l| l.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = parse_input("input.test");
        assert_eq!(input, vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn test_fish_increase_age() {
        let mut fish = Fish::new(1);

        let new_fish = fish.age();
        assert_eq!(fish.age, 0);
        assert_eq!(new_fish, false);

        let new_fish = fish.age();
        assert_eq!(fish.age, 6);
        assert_eq!(new_fish, true);
    }

    #[test]
    fn test_school_increase_age() {
        let mut school = School::new(vec![1, 2, 3]);
        school.age();
        assert_eq!(school.fish[0].age, 0);
        assert_eq!(school.fish[1].age, 1);
        assert_eq!(school.fish[2].age, 2);
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(find_school_size_at_time("input.test", 80), 5934);
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(find_school_size_at_time("input", 80), 359344);
    }
}
