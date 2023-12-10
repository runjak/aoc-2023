use std::{collections::HashMap, error::Error, fs};

type Coordinate = (i32, i32);
type Input = HashMap<Coordinate, char>;

fn parse_input(input: String) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| -> Vec<(Coordinate, char)> {
            line.chars()
                .enumerate()
                .map(|(x, char)| -> (Coordinate, char) {
                    ((x.try_into().unwrap(), y.try_into().unwrap()), char)
                })
                .collect()
        })
        .collect()
}

fn find_start(input: &Input) -> Coordinate {
    for (coordinate, symbol) in input.iter() {
        if 'S' == *symbol {
            return coordinate.to_owned();
        }
    }

    todo!("Unreachable by problem definition.");
}

// One sided transition possibility, may return invalid coordinates.
fn reachable(input: &Input, from: &Coordinate) -> Vec<Coordinate> {
    let symbol = input.get(from).unwrap_or(&'.');
    let (x, y) = from.to_owned();

    match symbol {
        '|' => Vec::from([(x, y - 1), (x, y + 1)]),
        '-' => Vec::from([(x - 1, y), (x + 1, y)]),
        'L' => Vec::from([(x, y - 1), (x + 1, y)]),
        'J' => Vec::from([(x, y - 1), (x - 1, y)]),
        '7' => Vec::from([(x, y + 1), (x - 1, y)]),
        'F' => Vec::from([(x, y + 1), (x + 1, y)]),
        '.' => Vec::new(),
        'S' => Vec::from([(x, y - 1), (x - 1, y), (x, y + 1), (x + 1, y)]),
        _ => Vec::new(),
    }
}

// Two-sided transition possibility
fn connected(input: &Input, from: &Coordinate) -> Vec<Coordinate> {
    reachable(input, from)
        .iter()
        .filter(|next| {
            let next_nexts = reachable(input, next);

            next_nexts.iter().any(|next_next| next_next == from)
        })
        .map(|c| c.to_owned())
        .collect()
}

type StepsTo = HashMap<Coordinate, i32>;

fn flood_fill(input: &Input, from: &Coordinate) -> StepsTo {
    let mut steps_to = HashMap::from([(from.to_owned(), 0)]);
    let mut nexts = connected(input, from);
    let mut steps_so_far = 1;

    while !&nexts.is_empty() {
        let mut over_nexts: Vec<Coordinate> = Vec::new();

        for next in nexts {
            if !steps_to.contains_key(&next) {
                steps_to.insert(next.to_owned(), steps_so_far);
                over_nexts.append(&mut connected(input, &next));
            }
        }
        steps_so_far += 1;

        // We could consider filtering the over_nexts here to not contain stuff already in steps_to
        nexts = over_nexts;
    }

    steps_to
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/10/example-1.txt", "./inputs/10/example-2.txt"]; //, "./inputs/10/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        let start = find_start(&input);
        let steps_to = flood_fill(&input, &start);

        let max_steps = steps_to.values().max().unwrap();
        println!("Max steps are {}", max_steps);
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented.");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("10-1:");
    first()?;
    println!("10-2:");
    second()?;

    Ok(())
}
