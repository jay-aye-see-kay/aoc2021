use std::collections::HashMap;
use std::fs;

// shorthand for creating a hashmap like vec![]
macro_rules! map {
    ($( $t: expr),*) => {{
         let mut map = HashMap::new();
         $( map.insert($t.0, $t.1); )*
         map
    }}
}

#[derive(Debug, PartialEq)]
enum Chunk {
    /// chunk is valid but Incomplete, stores the brackets required to complete the chunk
    Incomplete(Vec<char>),
    /// chuck cannot be completed, stores the (expected, found) brackets
    Invalid((char, char)),
}
impl Chunk {
    fn calc_points(&self) -> i64 {
        match self {
            Chunk::Incomplete(completions) => {
                let points_table = map![(')', 1), (']', 2), ('}', 3), ('>', 4)];
                completions
                    .iter()
                    .fold(0, |sum, completion| sum * 5 + points_table[completion])
            }
            Chunk::Invalid((_, found)) => {
                map![(')', 3), (']', 57), ('}', 1197), ('>', 25137)][found]
            }
        }
    }
}

fn main() {
    println!("part 1: {}", part_1("input"));
    println!("part 2: {}", part_2("input"));
}

/// Considering on the invalid chunks calculate the invalid score for each chuck and return the sum
fn part_1(filename: &str) -> i64 {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .filter_map(|line| match parse_to_chunk(line) {
            Chunk::Incomplete(_) => None,
            chunk => Some(chunk.calc_points()),
        })
        .sum()
}

/// Ignoring the invalid chunks, calculate the completion points for each chuck, the return the
/// median score
fn part_2(filename: &str) -> i64 {
    let mut scores: Vec<_> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .filter_map(|line| match parse_to_chunk(line) {
            Chunk::Invalid(_) => None,
            chunk => Some(chunk.calc_points()),
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

/// flip an open bracket to return its closing pair
fn flip_bracket(char: &char) -> char {
    if ['[', '{', '<'].contains(char) {
        (*char as u8 + 2) as char // these bracket pairs are 2 apart in the ascii table
    } else if *char == '(' {
        (*char as u8 + 1) as char // round bracket pair are next to each other
    } else {
        panic!("invalid bracket")
    }
}

fn validate_char(found: char, stack: &mut Vec<char>) -> Option<(char, char)> {
    let expected = flip_bracket(&stack.pop().unwrap());
    if found != expected {
        Some((expected, found))
    } else {
        None
    }
}

fn parse_to_chunk(line: &str) -> Chunk {
    let mut stack: Vec<char> = Vec::new();
    for char in line.chars() {
        if ['(', '[', '{', '<'].contains(&char) {
            stack.push(char)
        } else if let Some(invalid) = validate_char(char, &mut stack) {
            return Chunk::Invalid(invalid);
        }
    }
    Chunk::Incomplete(stack.iter().rev().map(flip_bracket).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_brackets() {
        let chunk_test_cases = vec![
            ("{([(<{}[<>[]}>{[]{[(<()>", Chunk::Invalid((']', '}'))),
            ("[[<[([]))<([[{}[[()]]]", Chunk::Invalid((']', ')'))),
            ("[{[{({}]{}}([{[{{{}}([]", Chunk::Invalid((')', ']'))),
            ("[<(<(<(<{}))><([]([]()", Chunk::Invalid(('>', ')'))),
            ("<{([([[(<>()){}]>(<<{{", Chunk::Invalid((']', '>'))),
            (
                "[({(<(())[]>[[{[]{<()<>>",
                Chunk::Incomplete("}}]])})]".chars().collect()),
            ),
            (
                "[(()[<>])]({[<{<<[]>>(",
                Chunk::Incomplete(")}>]})".chars().collect()),
            ),
            (
                "(((({<>}<{<{<>}{[]{[]{}",
                Chunk::Incomplete("}}>}>))))".chars().collect()),
            ),
            (
                "{<[[]]>}<{[{[{[]{()[[[]",
                Chunk::Incomplete("]]}}]}]}>".chars().collect()),
            ),
            (
                "<{([{{}}[<[[[<>{}]]]>[]]",
                Chunk::Incomplete("])}>".chars().collect()),
            ),
        ];
        chunk_test_cases.iter().for_each(|(line, expected)| {
            assert_eq!(parse_to_chunk(line), *expected);
        });
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1("input.test"), 26397);
    }

    #[test]
    fn test_part_1_real() {
        assert_eq!(part_1("input"), 358737);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2("input.test"), 288957);
    }

    #[test]
    fn test_part_2_real() {
        assert_eq!(part_2("input"), 4329504793);
    }
}
