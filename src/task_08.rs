use regex::Regex;
use std::{collections::HashMap, error::Error, fs};

#[derive(Debug)]
enum Step {
    Left,
    Right,
}

type Path = Vec<Step>;
type Graph = HashMap<String, (String, String)>;

#[derive(Debug)]
struct Input {
    path: Path,
    graph: Graph,
}

fn parse_path(line: String) -> Path {
    line.chars()
        .filter_map(|c| match c {
            'L' => Some(Step::Left),
            'R' => Some(Step::Right),
            _ => None,
        })
        .collect()
}

fn parse_graph(graph: String) -> Graph {
    let line_regex = Regex::new(r"(?<from>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();

    graph
        .lines()
        .filter_map(|line| -> Option<(String, (String, String))> {
            let captures = line_regex.captures(line)?;

            let from = captures.name("from")?.as_str();
            let left = captures.name("left")?.as_str();
            let right = captures.name("right")?.as_str();

            Some((from.to_string(), (left.to_string(), right.to_string())))
        })
        .collect()
}

fn parse_input(contents: String) -> Option<Input> {
    let blocks = contents.split("\n\n").collect::<Vec<_>>();
    let [path, graph] = blocks.as_slice() else {
        return None;
    };

    Some(Input {
        path: parse_path(path.to_string()),
        graph: parse_graph(graph.to_string()),
    })
}

fn apply_step(graph: &Graph, node: &String, step: &Step) -> String {
    let (left, right) = graph.get(node).unwrap();

    match step {
        Step::Left => left.to_string(),
        Step::Right => right.to_string(),
    }
}

fn travel(input: Input) -> i32 {
    let start = "AAA".to_string();
    let end = "ZZZ".to_string();

    let mut current = start;
    let mut steps = 0;

    for step in input.path.iter().cycle() {
        if current == end {
            break;
        }

        current = apply_step(&input.graph, &current, step);
        steps += 1;
    }

    return steps;
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = [
        "./inputs/08/example-1.txt",
        "./inputs/08/example-2.txt",
        "./inputs/08/input.txt",
    ];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let input = parse_input(contents).unwrap();

        let steps = travel(input);
        println!("Total setps: {}", steps);
    }

    Ok(())
}

fn start_nodes(input: &Input) -> Vec<String> {
    let start_node = Regex::new(r"\w\w[aA]$").unwrap();

    input
        .graph
        .keys()
        .filter(|key| start_node.is_match(&key))
        .map(|x| x.to_string())
        .collect()
}

fn is_finish_node(node: &str) -> bool {
    node.ends_with("Z")
}

fn ghost_travel(input: &Input, start: &String) -> i32 {
    let mut current = start.to_owned();
    let mut steps = 0;

    for step in input.path.iter().cycle() {
        if is_finish_node(&current) {
            break;
        }

        current = apply_step(&input.graph, &current, step);
        steps += 1;
    }

    steps
}

// credit to https://gist.github.com/victor-iyi/8a84185c1d52419b0d4915a648d5e3e1
fn gcd(mut n: i128, mut m: i128) -> i128 {
    assert!(n > 0 && m > 0);

    while m != 0 {
        if m < n {
            std::mem::swap(&mut n, &mut m);
        }

        m %= n;
    }

    n
}

// recalling https://github.com/runjak/aoc-2019/blob/master/src/12.ts
fn lcm(values: &Vec<i128>) -> i128 {
    assert!(values.len() > 0);

    let mut values = values.iter();
    let first = values.next().unwrap();

    values.fold(*first, |acc, v| {
        let v = *v;
        acc * v / gcd(acc, v)
    })
}

fn ghost_travels(input: Input) -> i128 {
    let starts = start_nodes(&input);

    let cycle_lengths: Vec<i128> = starts
        .iter()
        .map(|start| ghost_travel(&input, start) as i128)
        .collect();

    println!("cycle_lenghts: {:?}", cycle_lengths);

    lcm(&cycle_lengths)
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = [
        "./inputs/08/example-1.txt",
        "./inputs/08/example-2.txt",
        "./inputs/08/example-3.txt",
        "./inputs/08/input.txt",
    ];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let input = parse_input(contents).unwrap();

        let steps = ghost_travels(input);
        println!("Total setps: {}", steps);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("08-1:");
    first()?;
    println!("08-2:");
    second()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use crate::task_08::ghost_travels;

    use super::parse_input;

    #[test]
    fn ghost_travel_should_work_as_example_3() -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string("./inputs/08/example-3.txt")?;
        let input = parse_input(contents).unwrap();

        assert_eq!(ghost_travels(input), 6);

        Ok(())
    }
}
