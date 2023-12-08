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

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/08/example-1.txt", "./inputs/08/example-2.txt"]; //, "./inputs/08/input.txt"];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let input = parse_input(contents);

        println!("Got input: {:?}", input);

        // break;
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("08-1:");
    first()?;
    println!("08-2:");
    second()?;

    Ok(())
}
