use std::{collections::HashMap, error::Error, fs};

fn is_mirror(symbol: &char) -> bool {
    symbol == &'/' || symbol == &'\\'
}

fn is_splitter(symbol: &char) -> bool {
    symbol == &'-' || symbol == &'|'
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Position = (i32, i32);
type Field = HashMap<Position, char>;

fn parse_input(input: String) -> Field {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| -> Vec<(Position, char)> {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((i32::try_from(x).unwrap(), i32::try_from(y).unwrap()), c))
                .collect()
        })
        .collect()
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/16/example-1.txt", "./inputs/16/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        println!("Got input: {:?}", input);

        break;
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("16-1:");
    first()?;
    println!("16-2:");
    second()?;

    Ok(())
}
