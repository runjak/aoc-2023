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

fn filter_accepted(input: &Input) -> Vec<Part> {
    let catalog: HashMap<String, &Workflow> = input
        .workflows
        .iter()
        .map(|workflow| -> (String, &Workflow) { (workflow.name.to_string(), workflow) })
        .collect();

    let mut accepted: Vec<Part> = Vec::new();

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
            _ => {}
        }
    }

    accepted
}

fn score_sorted_parts(accepted_parts: &Vec<Part>) -> i32 {
    accepted_parts
        .iter()
        .map(|p| -> i32 { p.x + p.m + p.a + p.s })
        .sum::<i32>()
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/19/example-1.txt", "./inputs/19/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        let sorted_parts = filter_accepted(&input);
        let score = score_sorted_parts(&sorted_parts);

        println!("Score is: {}", score);
    }

    Ok(())
}

type Range = (i32, i32);

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

fn get_range_attribute(part_range: &PartRange, attribute: &Attribute) -> Range {
    match attribute {
        Attribute::X => part_range.x,
        Attribute::M => part_range.m,
        Attribute::A => part_range.a,
        Attribute::S => part_range.s,
    }
}

fn set_range_attribute(part_range: &PartRange, attribute: &Attribute, range: &Range) -> PartRange {
    let mut part_range = *part_range;

    match attribute {
        Attribute::X => part_range.x = *range,
        Attribute::M => part_range.m = *range,
        Attribute::A => part_range.a = *range,
        Attribute::S => part_range.s = *range,
    }

    part_range
}

fn ranges_from_comparison(
    range: &Range,
    value: i32,
    comparison: &char,
) -> (Option<Range>, Option<Range>) {
    /*
    Return value:
    First range: the one that is accepted for the value and comparison.
    Second range: the one that is rejected for the value and comparison.
    */
    if comparison == &'>' {
        if range.0 > value {
            // Whole range bigger than value
            return (Some(*range), None);
        } else if value > range.1 {
            // Value bigger than whole range
            return (None, Some(*range));
        } else {
            // Value somewhere in range
            return (Some((value + 1, range.1)), Some((range.0, value)));
        }
    } else {
        // Assume comparison of '<'
        if range.1 < value {
            // Whole range smaller than value
            return (Some(*range), None);
        } else if value < range.0 {
            // Value smaller than whole range
            return (None, Some(*range));
        } else {
            // Value somewhere in range
            return (Some((range.0, value - 1)), Some((value, range.1)));
        }
    }
}

fn apply_range_workflow(
    workflow: &Workflow,
    current_range: &PartRange,
) -> Vec<(WorkflowResult, PartRange)> {
    let mut current_range = *current_range;

    let mut results: Vec<(WorkflowResult, PartRange)> = Vec::new();

    for rule in workflow.rules.iter() {
        match rule {
            Rule::Conditional {
                attribute,
                comparison,
                value,
                name,
            } => {
                let range = get_range_attribute(&current_range, attribute);
                let (accepted, rejected) = ranges_from_comparison(&range, *value, comparison);

                if let Some(accepted) = accepted {
                    results.push((
                        workflow_result_from_string(name),
                        set_range_attribute(&current_range, attribute, &accepted),
                    ));
                }

                let Some(rejected) = rejected else {
                    break;
                };
                current_range = set_range_attribute(&current_range, attribute, &rejected);
            }
            Rule::Default(label) => {
                results.push((workflow_result_from_string(label), current_range));
                break;
            }
        }
    }

    results
}

fn filter_accepted_ranges(input: &Input) -> Vec<PartRange> {
    let catalog: HashMap<String, &Workflow> = input
        .workflows
        .iter()
        .map(|workflow| -> (String, &Workflow) { (workflow.name.to_string(), workflow) })
        .collect();

    let default_part_range = PartRange {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };

    let mut leads: Vec<(WorkflowResult, PartRange)> = Vec::from([(
        WorkflowResult::SeeOther("in".to_string()),
        default_part_range,
    )]);

    let mut accepted_part_ranges: Vec<PartRange> = Vec::new();

    while let Some(lead) = leads.pop() {
        match lead.0 {
            WorkflowResult::SeeOther(workflow_name) => {
                let Some(workflow) = catalog.get(&workflow_name) else {
                    continue;
                };

                leads.append(&mut apply_range_workflow(*workflow, &lead.1));
            }
            WorkflowResult::Accept => accepted_part_ranges.push(lead.1),
            WorkflowResult::Reject => (),
        }
    }

    accepted_part_ranges
}

type Combinations = i128;

fn score_part_ranges(part_ranges: &Vec<PartRange>) -> Combinations {
    part_ranges
        .iter()
        .map(|part_range| -> Combinations {
            let ranges = [part_range.x, part_range.m, part_range.a, part_range.s].to_vec();

            ranges
                .iter()
                .map(|(from, to)| -> Combinations { Combinations::from(to - from + 1) })
                .product()
        })
        .sum::<Combinations>()
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/19/example-1.txt", "./inputs/19/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        let accepted_ranges = filter_accepted_ranges(&input);
        let score = score_part_ranges(&accepted_ranges);

        println!("Score is: {}", score);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("19-1:");
    first()?;
    println!("19-2:");
    second()?;

    Ok(())
}
