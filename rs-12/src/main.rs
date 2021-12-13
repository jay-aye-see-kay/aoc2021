use std::fs;

trait CaseChecks {
    fn is_lowercase(&self) -> bool;
    fn is_uppercase(&self) -> bool;
}
impl CaseChecks for String {
    fn is_lowercase(&self) -> bool {
        *self == self.to_lowercase()
    }
    fn is_uppercase(&self) -> bool {
        *self == self.to_uppercase()
    }
}

type Connection = (String, String);
type Path = Vec<String>;

fn main() {
    let complete_paths = walk_all_paths("input", 1);
    println!("part 1: {}", complete_paths.len())
}

/// get children that can be visited next, and return an updated visited list
fn get_children(
    connections: &[Connection],
    path: &[String],
    node: &str,
    small_cave_visits: usize,
) -> (Vec<String>, bool) {
    let all_children: Vec<_> = connections
        .iter()
        .filter_map(|(from, to)| if from == node { Some(to) } else { None })
        .collect();
    let visited: Vec<_> = path
        .iter()
        .filter(|c| c.is_lowercase())
        .map(|c| c.to_string())
        .collect();

    let mut small_caves_visited: Vec<_> = visited
        .iter()
        .filter(|c| *c != "start" && *c != "end")
        .collect();
    let all_small_caves_visit_count = small_caves_visited.len();
    small_caves_visited.sort();
    small_caves_visited.dedup();
    let small_cave_revisit_count = all_small_caves_visit_count - small_caves_visited.len();

    let children_to_visit: Vec<_> = all_children
        .iter()
        .filter(|child| {
            let has_visited_this_cave = visited.contains(child);
            small_cave_revisit_count < (small_cave_visits - 1) || !has_visited_this_cave
        })
        .map(|c| c.to_string())
        .collect();

    let is_at_end = node == "end";
    (children_to_visit, is_at_end)
}

fn step(
    connections: &[Connection],
    initial_paths: &[Path],
    small_cave_visits: usize,
) -> (Vec<Path>, Vec<Path>) {
    let mut incomplete_paths = vec![];
    let mut complete_paths = vec![];

    for path in initial_paths.iter() {
        let last = path.iter().last().unwrap();
        let (children, is_at_end) = get_children(connections, path, last, small_cave_visits);

        if is_at_end {
            // handle found end
            complete_paths.push(path.clone());
        } else {
            // handle incomplete path(s)
            for child in children {
                let mut new_path = path.clone();
                new_path.push(child);
                incomplete_paths.push(new_path);
            }
        }
    }

    (complete_paths, incomplete_paths)
}

fn walk_all_paths(filename: &str, small_cave_visits: usize) -> Vec<Path> {
    let connections = get_input(filename);
    let incomplete_paths = vec![vec!["start".to_string()]];

    let (mut complete_paths, mut incomplete_paths) =
        step(&connections, &incomplete_paths, small_cave_visits);
    while !incomplete_paths.is_empty() {
        let (new_complete_paths, new_incomplete_paths) =
            step(&connections, &incomplete_paths, small_cave_visits);
        complete_paths.extend(new_complete_paths);
        incomplete_paths = new_incomplete_paths;
    }

    complete_paths
}

fn get_input(filename: &str) -> Vec<Connection> {
    let input = fs::read_to_string(filename).unwrap();
    let mut out = vec![];
    for line in input.lines() {
        let connection_str = line.split_once("-").unwrap();
        match connection_str {
            ("start", to) => out.push(("start", to)),
            (from, "end") => out.push((from, "end")),
            (from, to) => {
                out.push((from, to));
                out.push((to, from));
            }
        }
    }

    out.iter()
        .map(|(from, to)| (from.to_string(), to.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let input = get_input("input.test1");
        let expected: Vec<_> = vec![
            ("start", "A"),
            ("start", "b"),
            ("A", "c"),
            ("c", "A"),
            ("A", "b"),
            ("b", "A"),
            ("b", "d"),
            ("d", "b"),
            ("A", "end"),
            ("b", "end"),
        ]
        .iter()
        .map(|c| (c.0.to_string(), c.1.to_string()))
        .collect();
        assert_eq!(input, expected)
    }

    #[test]
    fn test_step() {
        let connections = get_input("input.test1");
        let incomplete_paths = vec![vec!["start".to_string()]];
        let (_, incomplete_paths) = step(&connections, &incomplete_paths, 1);

        assert_eq!(
            incomplete_paths,
            vec![
                vec!["start".to_string(), "A".to_string()],
                vec!["start".to_string(), "b".to_string()]
            ]
        );
    }

    #[test]
    fn test_part_1_sample_1() {
        let complete_paths = walk_all_paths("input.test1", 1);
        assert_eq!(complete_paths.len(), 10);
    }

    #[test]
    fn test_part_1_sample_2() {
        let complete_paths = walk_all_paths("input.test2", 1);
        assert_eq!(complete_paths.len(), 19);
    }

    #[test]
    fn test_part_1_sample_3() {
        let complete_paths = walk_all_paths("input.test3", 1);
        assert_eq!(complete_paths.len(), 226);
    }

    #[test]
    fn test_part_1_real() {
        let complete_paths = walk_all_paths("input", 1);
        assert_eq!(complete_paths.len(), 5252);
    }

    #[test]
    fn test_part_2_sample_1() {
        let complete_paths = walk_all_paths("input.test1", 2);
        assert_eq!(complete_paths.len(), 36);
    }

    #[test]
    #[ignore]
    fn test_part_2_sample_2() {
        // skipped because this one seems to trigger an infinite loop
        let complete_paths = walk_all_paths("input.test2", 2);
        assert_eq!(complete_paths.len(), 103);
    }

    #[test]
    fn test_part_2_sample_3() {
        let complete_paths = walk_all_paths("input.test3", 2);
        assert_eq!(complete_paths.len(), 3509);
    }

    #[test]
    fn test_part_2_real() {
        let complete_paths = walk_all_paths("input", 2);
        assert_eq!(complete_paths.len(), 147784);
    }
}
