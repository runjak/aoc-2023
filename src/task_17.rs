use std::{collections::HashMap, error::Error, fs};

type Position = (i32, i32);
type Field = HashMap<Position, i32>;

fn parse_input(input: String) -> Field {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| -> Vec<(Position, i32)> {
            line.chars()
                .enumerate()
                .map(|(x, c)| -> (Position, i32) {
                    let position: Position = (i32::try_from(x).unwrap(), i32::try_from(y).unwrap());
                    let value = c.to_string().parse::<i32>().unwrap();

                    (position, value)
                })
                .collect()
        })
        .collect()
}

fn get_max_position(field: &Field) -> Position {
    let max_x = field.keys().map(|(x, _)| *x).max().unwrap_or(0);
    let max_y = field.keys().map(|(_, y)| *y).max().unwrap_or(0);

    (max_x, max_y)
}

fn next_positions((x, y): &Position, (max_x, max_y): &Position) -> Vec<Position> {
    [(x - 1, *y), (x + 1, *y), (*x, y - 1), (*x, y + 1)]
        .into_iter()
        .filter(|(x, y)| x >= &0 && y >= &0 && x <= max_x && y <= max_y)
        .collect()
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/17/example-1.txt", "./inputs/17/input.txt"];

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
    println!("17-1:");
    first()?;
    println!("17-2:");
    second()?;

    Ok(())
}
