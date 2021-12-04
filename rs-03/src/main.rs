use std::fs;

// read file to lines
// create vector of length of lines
// read lines and increment
fn main() {
    println!("part 1: {}", part_1("input"));
}

fn part_1(filename: &str) -> i32 {
    let mut line_count = 0;
    let file = fs::read_to_string(filename).unwrap();
    let mut counts: Vec<i32> = Vec::new();

    for (line_idx, line) in file.lines().enumerate() {
        line_count += 1;
        for (char_idx, char) in line.chars().enumerate() {
            if line_idx == 0 {
                counts.push(0);
            }
            if char == '1' {
                counts[char_idx] += 1;
            }
        }
    }

    let (gamma, epsilon) = counts.iter().fold((0, 0), |(gamma, epsilon), c| {
        let mut new_gamma = gamma << 1;
        let mut new_epsilon = epsilon << 1;
        if *c > line_count / 2 {
            new_gamma = new_gamma | 1;
        } else {
            new_epsilon = new_epsilon | 1;
        }
        return (new_gamma, new_epsilon);
    });

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1("input.test"), 198);
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(part_1("input"), 3912944);
    }
}
