use std::{error::Error, fs};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Dig {
    direction: Direction,
    length: i32,
    color: String,
}

fn parse_input(input: String) -> Vec<Dig> {
    input
        .lines()
        .filter_map(|line| -> Option<Dig> {
            let parts = line.split(" ").collect::<Vec<_>>();
            let [direction, length, color] = parts.as_slice() else {
                return None;
            };

            let direction = (match *direction {
                "U" => Some(Direction::Up),
                "D" => Some(Direction::Down),
                "L" => Some(Direction::Left),
                "R" => Some(Direction::Right),
                _ => None,
            })?;

            let length = length.parse::<i32>().ok()?;

            let color = color[1..color.len() - 1].to_string();

            Some(Dig {
                direction,
                length,
                color,
            })
        })
        .collect()
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/18/example-1.txt", "./inputs/18/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

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
    println!("18-1:");
    first()?;
    println!("18-2:");
    second()?;

    Ok(())
}
