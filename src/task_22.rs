use std::{error::Error, fs};

type N = i32;
type Position = (N, N, N);
type Brick = (Position, Position);

fn parse_position(position: &str) -> Option<Position> {
    let parts = position.split(",").collect::<Vec<_>>();

    match parts.as_slice() {
        [x, y, z] => Some((
            x.parse::<N>().ok()?,
            y.parse::<N>().ok()?,
            z.parse::<N>().ok()?,
        )),
        _ => None,
    }
}

fn parse_bricks(input: String) -> Vec<Brick> {
    input
        .lines()
        .filter_map(|line| -> Option<Brick> {
            let (from, to) = line.split_once("~")?;

            Some((parse_position(from)?, parse_position(to)?))
        })
        .collect()
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/22/example-1.txt", "./inputs/22/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_bricks(input);

        println!("Got input:\n{:?}", input);

        break;
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("22-1:");
    first()?;
    println!("22-2:");
    second()?;

    Ok(())
}
