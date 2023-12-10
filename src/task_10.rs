use std::{collections::HashMap, error::Error, fs};

type Coordinate = (usize, usize);
type Input = HashMap<Coordinate, char>;

fn parse_input(input: String) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| -> Vec<(Coordinate, char)> {
            line.chars()
                .enumerate()
                .map(|(x, char)| -> (Coordinate, char) { ((x, y), char) })
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

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/10/example-1.txt", "./inputs/10/example-2.txt"]; //, "./inputs/10/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        println!("Got input:\n{:?}", input);

        let start = find_start(&input);
        println!("Start is: {:?}", start);
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
