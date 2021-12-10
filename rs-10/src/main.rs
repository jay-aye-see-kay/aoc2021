use std::fs;

fn main() {
    println!("part 1: {}", part_1("input"));
    println!("part 2: {}", part_2("input"));
}

fn part_1(filename: &str) -> i64 {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| get_points_of_first_invalid_char(line))
        .sum()
}

fn part_2(filename: &str) -> i64 {
    let input = fs::read_to_string(filename).unwrap();
    let mut scores: Vec<_> = input
        .lines()
        .map(|line| validate_brackets(line))
        .map(|res| match res {
            BracketResult::Valid(completion) => completion_score(&completion),
            BracketResult::Invalid(_) => 0,
        })
        .filter(|score| score > &0)
        .collect();

    scores.sort_unstable();
    println!("scores: {:?}", scores);
    let res = scores[scores.len() / 2];
    res
}

fn completion_score(completions: &Vec<char>) -> i64 {
    completions.iter().fold(0, |sum, completion| {
        let score = match completion {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!(),
        };
        sum * 5 + score
    })
}

fn get_points_of_first_invalid_char(line: &str) -> i64 {
    let res = validate_brackets(line);
    match res {
        BracketResult::Invalid((_, ')')) => 3,
        BracketResult::Invalid((_, ']')) => 57,
        BracketResult::Invalid((_, '}')) => 1197,
        BracketResult::Invalid((_, '>')) => 25137,
        BracketResult::Invalid((_, _)) => 0,
        BracketResult::Valid(_) => 0,
    }
}

fn flip(char: &char) -> char {
    match char {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        _ => panic!("invalid bracket"),
    }
}

#[derive(Debug, PartialEq)]
enum BracketResult {
    Valid(Vec<char>),
    Invalid((char, char)),
}

fn validate_brackets(line: &str) -> BracketResult {
    let mut stack: Vec<char> = Vec::new();

    for char in line.chars() {
        // TODO handle the case where the stack is empty?
        match char {
            '(' => stack.push('('),
            ')' => {
                let found = ')';
                let from_stack = stack.pop().or(Some('x')).unwrap();
                let expected = flip(&from_stack);
                if found != expected {
                    return BracketResult::Invalid((expected, found));
                }
            }
            '[' => stack.push('['),
            ']' => {
                let found = ']';
                let from_stack = stack.pop().or(Some('x')).unwrap();
                let expected = flip(&from_stack);
                if found != expected {
                    return BracketResult::Invalid((expected, found));
                }
            }
            '{' => stack.push('{'),
            '}' => {
                let found = '}';
                let from_stack = stack.pop().or(Some('x')).unwrap();
                let expected = flip(&from_stack);
                if found != expected {
                    return BracketResult::Invalid((expected, found));
                }
            }
            '<' => stack.push('<'),
            '>' => {
                let found = '>';
                let from_stack = stack.pop().or(Some('x')).unwrap();
                let expected = flip(&from_stack);
                if found != expected {
                    return BracketResult::Invalid((expected, found));
                }
            }
            _ => {
                println!("unexpected char: {}", char);
            }
        }
    }
    BracketResult::Valid(stack.iter().rev().map(|c| flip(c)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_brackets() {
        vec![
            (
                "[({(<(())[]>[[{[]{<()<>>",
                BracketResult::Valid(vec!['}', '}', ']', ']', ')', '}', ')', ']']),
            ),
            (
                "[(()[<>])]({[<{<<[]>>(",
                BracketResult::Valid(vec![')', '}', '>', ']', '}', ')']),
            ),
            (
                "{([(<{}[<>[]}>{[]{[(<()>",
                BracketResult::Invalid((']', '}')),
            ),
            (
                "(((({<>}<{<{<>}{[]{[]{}",
                BracketResult::Valid(vec!['}', '}', '>', '}', '>', ')', ')', ')', ')']),
            ),
            ("[[<[([]))<([[{}[[()]]]", BracketResult::Invalid((']', ')'))),
            (
                "[{[{({}]{}}([{[{{{}}([]",
                BracketResult::Invalid((')', ']')),
            ),
            (
                "{<[[]]>}<{[{[{[]{()[[[]",
                BracketResult::Valid(vec![']', ']', '}', '}', ']', '}', ']', '}', '>']),
            ),
            ("[<(<(<(<{}))><([]([]()", BracketResult::Invalid(('>', ')'))),
            ("<{([([[(<>()){}]>(<<{{", BracketResult::Invalid((']', '>'))),
            (
                "<{([{{}}[<[[[<>{}]]]>[]]",
                BracketResult::Valid(vec![']', ')', '}', '>']),
            ),
        ]
        .iter()
        .for_each(|(line, expected)| {
            assert_eq!(validate_brackets(line), *expected);
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
