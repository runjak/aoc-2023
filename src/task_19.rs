use regex::Regex;
use std::{collections::HashMap, error::Error, fs};

#[derive(Debug)]
enum Attribute {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Rule {
    Conditional {
        attribute: Attribute,
        comparison: char,
        value: i32,
        name: String,
    },
    Default(String),
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[derive(Debug)]
struct Input {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

fn parse_rule(input: &str) -> Option<Rule> {
    if !input.contains(":") {
        return Some(Rule::Default(input.to_string()));
    }

    let conditional_regex =
        Regex::new(r"(?<attribute>[xmas])(?<comparison>[><])(?<value>\d+):(?<name>\w+)").unwrap();

    let captures = conditional_regex.captures(input)?;

    let attribute = captures.name("attribute")?.as_str();
    let attribute = match attribute {
        "x" => Some(Attribute::X),
        "m" => Some(Attribute::M),
        "a" => Some(Attribute::A),
        "s" => Some(Attribute::S),
        _ => None,
    }?;

    let comparison = captures.name("comparison")?.as_str().chars().next()?;
    let value = captures.name("value")?.as_str().parse::<i32>().ok()?;
    let name = captures.name("name")?.as_str().to_string();

    Some(Rule::Conditional {
        attribute,
        comparison,
        value,
        name,
    })
}

fn parse_workflow(input: &str) -> Option<Workflow> {
    let workflow_regex = Regex::new(r"(?<name>\w+)\{(?<rules>.+)\}$").unwrap();
    let captures = workflow_regex.captures(input)?;

    let name = captures.name("name")?.as_str().to_string();

    let rules = captures.name("rules")?.as_str().to_string();
    let rules = rules
        .split(",")
        .filter_map(|rule| parse_rule(rule))
        .collect::<Vec<_>>();

    Some(Workflow { name, rules })
}

fn parse_part(input: &str) -> Option<Part> {
    let parts_regex = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
    let captures = parts_regex.captures(input)?;

    let x = captures.name("x")?.as_str().parse::<i32>().ok()?;
    let m = captures.name("m")?.as_str().parse::<i32>().ok()?;
    let a = captures.name("a")?.as_str().parse::<i32>().ok()?;
    let s = captures.name("s")?.as_str().parse::<i32>().ok()?;

    Some(Part { x, m, a, s })
}

fn parse_input(input: String) -> Input {
    let (workflows, parts) = input.split_once("\n\n").unwrap_or(("", ""));

    let workflows = workflows
        .lines()
        .filter_map(|line| parse_workflow(line))
        .collect::<Vec<_>>();

    let parts = parts
        .lines()
        .filter_map(|line| parse_part(line))
        .collect::<Vec<_>>();

    Input { workflows, parts }
}

#[derive(Debug, Clone, PartialEq)]
enum WorkflowResult {
    SeeOther(String),
    Accept,
    Reject,
}

fn workflow_result_from_string(label: &String) -> WorkflowResult {
    if label.as_str() == "A" {
        return WorkflowResult::Accept;
    } else if label.as_str() == "R" {
        return WorkflowResult::Reject;
    } else {
        return WorkflowResult::SeeOther(label.to_string());
    }
}

fn apply_workflow(workflow: &Workflow, part: &Part) -> WorkflowResult {
    for rule in workflow.rules.iter() {
        match rule {
            Rule::Conditional {
                attribute,
                comparison,
                value,
                name,
            } => {
                let part_value = match attribute {
                    Attribute::X => part.x,
                    Attribute::M => part.m,
                    Attribute::A => part.a,
                    Attribute::S => part.s,
                };

                if comparison == &'>' && part_value > *value {
                    return workflow_result_from_string(name);
                }

                if comparison == &'<' && part_value < *value {
                    return workflow_result_from_string(name);
                }
            }
            Rule::Default(label) => {
                return workflow_result_from_string(label);
            }
        }
    }

    WorkflowResult::Reject
}

struct SortedParts {
    accepted: Vec<Part>,
    rejected: Vec<Part>,
}

fn sort_parts(input: &Input) -> SortedParts {
    let catalog: HashMap<String, &Workflow> = input
        .workflows
        .iter()
        .map(|workflow| -> (String, &Workflow) { (workflow.name.to_string(), workflow) })
        .collect();

    let mut accepted: Vec<Part> = Vec::new();
    let mut rejected: Vec<Part> = Vec::new();

    for part in input.parts.iter() {
        let mut current_result: WorkflowResult = WorkflowResult::SeeOther("in".to_string());

        while let WorkflowResult::SeeOther(ref name) = current_result {
            let Some(current_workflow) = catalog.get(name) else {
                break;
            };

            current_result = apply_workflow(*current_workflow, part);
        }

        match current_result {
            WorkflowResult::Accept => {
                accepted.push(*part);
            }
            WorkflowResult::Reject => {
                rejected.push(*part);
            }
            WorkflowResult::SeeOther(_) => {}
        }
    }

    SortedParts { accepted, rejected }
}

fn score_sorted_parts(sorted_parts: &SortedParts) -> i32 {
    sorted_parts
        .accepted
        .iter()
        .map(|p| -> i32 { p.x + p.m + p.a + p.s })
        .sum::<i32>()
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/19/example-1.txt", "./inputs/19/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        let sorted_parts = sort_parts(&input);
        let score = score_sorted_parts(&sorted_parts);

        println!("Score is: {}", score);
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented.");
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("19-1:");
    first()?;
    println!("19-2:");
    second()?;

    Ok(())
}
