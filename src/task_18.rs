use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Color = String;

#[derive(Debug)]
struct Dig {
    direction: Direction,
    length: i32,
    color: Color,
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

type Position = (i32, i32);
type Field = HashMap<Position, Color>;

fn apply_direction((x, y): &Position, direction: &Direction) -> Position {
    match direction {
        Direction::Up => (*x, y - 1),
        Direction::Down => (*x, y + 1),
        Direction::Left => (x - 1, *y),
        Direction::Right => (x + 1, *y),
    }
}

fn dig_input(input: &Vec<Dig>) -> Field {
    let mut current_position: Position = (0, 0);
    let mut lagoon: Field = HashMap::from([(current_position, "".to_string())]);

    for dig in input {
        for _ in 0..dig.length {
            current_position = apply_direction(&current_position, &dig.direction);
            lagoon.insert(current_position, dig.color.to_string());
        }
    }

    lagoon
}

fn dig_interior(field: &Field) -> Field {
    let max_x = *field.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let max_y = *field.keys().map(|(_, y)| y).max().unwrap_or(&0);

    let mut field = field.clone();

    for y in 0..=max_y {
        let mut inside = false;

        for x in 0..=max_x {
            let current_position = (x, y);
            let previous_position = (x - 1, y);

            if inside {
                if field.contains_key(&previous_position) && !field.contains_key(&current_position)
                {
                    field.insert(current_position, "".to_string());
                } else {
                }
            } else {
            }

            let has_current = field.get(&current_position);
        }
    }

    todo!("Rest of the owl")
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

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, error::Error, fs};

    use super::{dig_input, parse_input, Position};

    #[test]
    fn initial_dig_from_input_should_match_example() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string("./inputs/18/example-1.txt")?;
        let input = parse_input(input);

        let actual = dig_input(&input).keys().map(|p| *p).collect::<HashSet<_>>();

        let expected = fs::read_to_string("./inputs/18/trench-1.txt")?;
        let expected = expected
            .lines()
            .enumerate()
            .flat_map(|(y, line)| -> Vec<Position> {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| -> Option<Position> {
                        if c != '#' {
                            return None;
                        }

                        Some((i32::try_from(x).unwrap(), i32::try_from(y).unwrap()))
                    })
                    .collect()
            })
            .collect::<HashSet<_>>();

        assert_eq!(actual, expected);

        Ok(())
    }
}
