use std::fs;
use std::str::FromStr;

fn main() {
    let school: School = fs::read_to_string("input.test").unwrap().parse().unwrap();
    println!("part 1: {}", school.size_at_time(80));
    println!("part 2: {}", school.size_at_time(256));
}

#[derive(Clone)]
struct School {
    fish_by_age: Vec<i64>,
}

impl FromStr for School {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let ages: Vec<i64> = input
            .trim()
            .split(",")
            .map(|l| l.parse().unwrap())
            .collect();
        Ok(School {
            fish_by_age: ages.iter().fold(vec![0; 9], |mut by_age, age| {
                by_age[*age as usize] += 1;
                by_age
            }),
        })
    }
}

impl School {
    fn tick(&mut self) {
        let giving_birth_count = self.fish_by_age.remove(0);
        self.fish_by_age.push(giving_birth_count);
        self.fish_by_age[6] += giving_birth_count;
    }

    fn size_at_time(&self, time: i64) -> i64 {
        let mut school = self.clone();
        for _ in 0..time {
            school.tick();
        }
        school.fish_by_age.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let school: School = fs::read_to_string("input.test").unwrap().parse().unwrap();
        assert_eq!(school.fish_by_age, vec![0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_part_1_sample() {
        let school: School = fs::read_to_string("input.test").unwrap().parse().unwrap();
        assert_eq!(school.size_at_time(80), 5934);
    }

    #[test]
    fn test_part_1_real() {
        let school: School = fs::read_to_string("input").unwrap().parse().unwrap();
        assert_eq!(school.size_at_time(80), 359344);
    }

    #[test]
    fn test_part_2_sample() {
        let school: School = fs::read_to_string("input.test").unwrap().parse().unwrap();
        assert_eq!(school.size_at_time(256), 26984457539);
    }

    #[test]
    fn test_part_2_real() {
        let school: School = fs::read_to_string("input").unwrap().parse().unwrap();
        assert_eq!(school.size_at_time(256), 1629570219571);
    }
}
