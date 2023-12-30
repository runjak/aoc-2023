use std::{collections::HashMap, error::Error, fs};

#[derive(Debug, Clone, Copy)]
enum SignalType {
    Low,
    High,
}

#[derive(Debug)]
struct Signal {
    from: String,
    signal_type: SignalType,
    to: String,
}

#[derive(Debug)]
enum Module {
    Broadcaster {
        name: String,
        outputs: Vec<String>,
    },
    FlipFlop {
        name: String,
        is_on: bool,
        outputs: Vec<String>,
    },
    Conjunction {
        name: String,
        inputs: HashMap<String, SignalType>,
        outputs: Vec<String>,
    },
}

fn parse_input(input: String) -> Vec<Module> {
    input
        .lines()
        .filter_map(|line| -> Option<Module> {
            let (name, outputs) = line.split_once(" -> ")?;
            let outputs = outputs
                .split(", ")
                .map(|output| output.to_string())
                .collect::<Vec<_>>();

            if name == "broadcaster" {
                return Some(Module::Broadcaster {
                    name: name.to_string(),
                    outputs,
                });
            }

            let prefix = &name[0..1];
            let name = name[1..].to_string();

            if prefix == "%" {
                return Some(Module::FlipFlop {
                    name,
                    is_on: false,
                    outputs,
                });
            }

            if prefix == "&" {
                return Some(Module::Conjunction {
                    name,
                    inputs: HashMap::new(),
                    outputs,
                });
            }

            None
        })
        .collect()
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = [
        "./inputs/20/example-1.txt",
        "./inputs/20/example-2.txt",
        // "./inputs/20/input.txt",
    ];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        println!("Parsed input:\n{:?}", input);
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("20-1:");
    first()?;
    println!("20-2:");
    second()?;

    Ok(())
}
