use std::fs;

// read file to lines
// create vector of length of lines
// read lines and increment
fn main() {
    println!("part 1: {}", part_1("input"));
    println!("part 2: {}", part_2("input"));
}

fn count_ones(file: &str) -> (i32, Vec<i32>) {
    let mut line_count = 0;
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
    return (line_count, counts);
}

fn part_1(filename: &str) -> i32 {
    let file = fs::read_to_string(filename).unwrap();
    let (line_count, counts) = count_ones(&file);

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

fn filter_by_position(to_keep: &char, at_position: &usize, file: &str) -> String {
    file.lines()
        .filter(|line| line.chars().nth(*at_position).unwrap() == *to_keep)
        .fold("".to_string(), |sum, line| {
            [sum, line.to_string(), "\n".to_string()].concat()
        })
}

fn most_common_at(line_count: i32, counts: &Vec<i32>, at_position: usize) -> char {
    if counts[at_position] as f64 >= line_count as f64 / 2.0 {
        '1'
    } else {
        '0'
    }
}

fn least_common_at(line_count: i32, counts: &Vec<i32>, at_position: usize) -> char {
    if counts[at_position] as f64 >= line_count as f64 / 2.0 {
        '0'
    } else {
        '1'
    }
}

fn binary_str_to_num(str: &str) -> i32 {
    str.chars().fold(0, |num, char| {
        let shifted = num << 1;
        match char {
            '1' => shifted | 1,
            '0' => shifted,
            '\n' => num,
            _ => panic!("unexpected char {} in binary string", char),
        }
    })
}

fn part_2(filename: &str) -> i32 {
    let mut file = fs::read_to_string(filename).unwrap();

    let mut considered_idx = 0;

    let oxygen_generator_rating;
    let co2_scrubber_rating;

    loop {
        let (line_count, counts) = count_ones(&file);
        if line_count == 1 {
            oxygen_generator_rating = binary_str_to_num(&file);
            break;
        }
        let most_common = most_common_at(line_count, &counts, considered_idx);
        file = filter_by_position(&most_common, &considered_idx, &file);
        considered_idx += 1;
    }

    let mut file = fs::read_to_string(filename).unwrap();
    let mut considered_idx = 0;
    loop {
        let (line_count, counts) = count_ones(&file);
        if line_count == 1 {
            co2_scrubber_rating = binary_str_to_num(&file);
            break;
        }
        let most_common = least_common_at(line_count, &counts, considered_idx);
        file = filter_by_position(&most_common, &considered_idx, &file);
        considered_idx += 1;
    }

    oxygen_generator_rating * co2_scrubber_rating
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

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2("input.test"), 230);
    }

    #[test]
    fn test_part_2_full() {
        assert_eq!(part_2("input"), 4996233);
    }
}
