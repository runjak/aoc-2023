use regex::Regex;
use std::{error::Error, fs};

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

#[derive(Debug)]
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

    Some(Part{ x, m, a, s })
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

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/19/example-1.txt", "./inputs/19/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        println!("Got input:\n{:?}", input);

        break;
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
