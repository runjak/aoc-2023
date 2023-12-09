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
    let finish_node = Regex::new(r"\w\w[zZ]$").unwrap();

    finish_node.is_match(node)
}

struct GhostCycle {
    // Length of the cycle.
    cycle_length: i32,
    // Steps to take until visiting the finish node the first time.
    offsets: Vec<i32>,
}

fn find_ghost_cycle(input: &Input, start_node: &String) -> GhostCycle {
    // The simulataneous ghost travels can be understood independently.
    // We're then looking for something like the lcm of the paths, but slightly worse.
    // That is because we need to incorporate two aspects:
    // 1: There may be an initial offset to a finish node.
    // 2: There may be multiple finish nodes visited in some cycle.
    // Idea:
    // For each finish node on the cycle we can produce an initial offset and a common cycle length.
    // Hence we need to produce a cycle length along with number of possible offsets to start with.

    // The cycle of steps, but with indices to identify their position in the path.
    // We have a cycle when we revisit a node with the same index as previously.
    // If we revisit a node but have a different index it is possible that the following steps will not be the same as before.
    let steps = input.path.iter().enumerate().cycle();

    struct VisitedAt {
        offset: i32,
        step_index: usize,
    }
    let mut history = HashMap::from([(
        start_node,
        VisitedAt {
            offset: 0,
            // We know that the last step of the path must've been the one leading to the start.
            // We may end up in a smaller cycle than one including the start,
            // but if we revisit the start we expect that to align with the last step.
            step_index: input.path.len() - 1,
        },
    )]);

    let mut current_node = start_node;
    let mut cycle_length = 0;
    for (step_index, step) in steps {
        // The proof of the pudding belongs here.
    }

    let offsets = history
        .iter()
        .filter(|(node, _)| is_finish_node(node))
        .map(|(_, visited_at)| visited_at.offset)
        .collect();

    GhostCycle {
        cycle_length,
        offsets,
    }
}

fn ghost_travel(input: Input) -> i32 {
    let starts = start_nodes(&input);
    let ghost_cycles = starts
        .iter()
        .map(|start_node| find_ghost_cycle(&input, start_node))
        .collect::<Vec<_>>();

    // FIXME this is where the real magic happens.
    !todo!()
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

        let steps = ghost_travel(input);
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

    use crate::task_08::ghost_travel;

    use super::parse_input;

    #[test]
    fn ghost_travel_should_work_as_example_3() -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string("./inputs/08/example-3.txt")?;
        let input = parse_input(contents).unwrap();

        assert_eq!(ghost_travel(input), 6);

        Ok(())
    }
}
