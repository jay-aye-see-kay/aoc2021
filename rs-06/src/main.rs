use std::fs;

fn main() {
    println!("part 1: {}", find_school_size_at_time("input", 80));
    println!("part 2: {}", find_school_size_at_time("input", 256));
}

struct School {
    fish_by_age: Vec<i64>,
}

impl School {
    fn new(ages: Vec<i64>) -> Self {
        Self {
            fish_by_age: ages.iter().fold(vec![0; 9], |mut by_age, age| {
                by_age[*age as usize] += 1;
                by_age
            }),
        }
    }

    fn tick(&mut self) {
        let giving_birth_count = self.fish_by_age.remove(0);
        self.fish_by_age.push(giving_birth_count);
        self.fish_by_age[6] += giving_birth_count;
    }
}

fn find_school_size_at_time(filename: &str, time: i64) -> i64 {
    let mut school = School::new(parse_input(filename));
    for _ in 0..time {
        school.tick();
    }
    school.fish_by_age.iter().sum()
}

fn parse_input(filename: &str) -> Vec<i64> {
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
    fn test_part_1_sample() {
        assert_eq!(find_school_size_at_time("input.test", 80), 5934);
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(find_school_size_at_time("input", 80), 359344);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(find_school_size_at_time("input.test", 256), 26984457539);
    }

    #[test]
    fn test_part_2_real() {
        assert_eq!(find_school_size_at_time("input", 256), 1629570219571);
    }
}
