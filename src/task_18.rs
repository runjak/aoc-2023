use std::{collections::HashSet, error::Error, fs};

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Color = String;

#[derive(Debug, PartialEq)]
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
type Trench = HashSet<Position>;

fn apply_direction((x, y): &Position, direction: &Direction) -> Position {
    match direction {
        Direction::Up => (*x, y - 1),
        Direction::Down => (*x, y + 1),
        Direction::Left => (x - 1, *y),
        Direction::Right => (x + 1, *y),
    }
}

fn dig_trench(input: &Vec<Dig>) -> Trench {
    let mut current_position: Position = (0, 0);
    let mut lagoon: Trench = HashSet::from([current_position]);

    for dig in input {
        for _ in 0..dig.length {
            current_position = apply_direction(&current_position, &dig.direction);
            lagoon.insert(current_position);
        }
    }

    lagoon
}

fn shift_positive(trench: &Trench) -> Trench {
    let min_x = trench.iter().map(|(x, _)| x).min().unwrap_or(&0);
    let min_y = trench.iter().map(|(_, y)| y).min().unwrap_or(&0);

    trench
        .iter()
        .map(|(x, y)| -> Position { (x - min_x, y - min_y) })
        .collect()
}

fn dig_interior(trench: &Trench) -> Trench {
    let trench = shift_positive(trench);

    let outside_x = *trench.iter().map(|(x, _)| x).max().unwrap_or(&0) + 1;
    let outside_y = *trench.iter().map(|(_, y)| y).max().unwrap_or(&0) + 1;

    let mut interior: Trench = (-1..=outside_x + 1)
        .flat_map(|x| -> Vec<Position> { (-1..=outside_y).map(|y| (x, y)).collect() })
        .collect();

    for trench_position in trench.iter() {
        interior.remove(trench_position);
    }

    let mut to_remove: Vec<Position> = Vec::from([(-1, -1)]);

    while let Some(position) = to_remove.pop() {
        interior.remove(&position);

        for d in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let next_position = apply_direction(&position, &d);
            if interior.contains(&next_position) {
                to_remove.push(next_position);
            }
        }
    }

    for trench_position in trench.iter() {
        interior.insert(*trench_position);
    }

    interior
}

#[allow(dead_code)]
fn interior_to_string(interior: &Trench) -> String {
    let max_x = *interior.iter().map(|(x, _)| x).max().unwrap_or(&0);
    let max_y = *interior.iter().map(|(_, y)| y).max().unwrap_or(&0);

    let lines = (0..=max_y)
        .map(|y| -> String {
            (0..=max_x)
                .map(|x| -> char {
                    if interior.contains(&(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    lines.join("\n")
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/18/example-1.txt", "./inputs/18/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        let trench = dig_trench(&input);
        let trench = dig_interior(&trench);

        let capacity = trench.len();

        println!("Capacity: {}", capacity);
    }

    Ok(())
}

fn interpret_colors(input: &Vec<Dig>) -> Vec<Dig> {
    input
        .iter()
        .filter_map(|dig| -> Option<Dig> {
            if !dig.color.starts_with("#") {
                return None;
            }

            let color = &dig.color[1..];
            if color.len() != 6 {
                return None;
            }

            let length = i32::from_str_radix(&color[0..color.len() - 1], 16).ok()?;

            let direction = match color.chars().last().unwrap_or('.') {
                '0' => Some(Direction::Right),
                '1' => Some(Direction::Down),
                '2' => Some(Direction::Left),
                '3' => Some(Direction::Up),
                _ => None,
            }?;

            Some(Dig {
                direction,
                length,
                color: dig.color.to_string(),
            })
        })
        .collect()
}

fn apply_dig((x, y): &Position, dig: &Dig) -> Position {
    let length = dig.length;

    match dig.direction {
        Direction::Up => (*x, y - length),
        Direction::Down => (*x, y + length),
        Direction::Left => (x - length, *y),
        Direction::Right => (x + length, *y),
    }
}

type Outline = Vec<Position>;

fn fast_dig(input: &Vec<Dig>) -> Outline {
    let mut current_position: Position = (0, 0);
    let mut trench: Outline = Vec::from([current_position]);

    for dig in input {
        current_position = apply_dig(&current_position, dig);
        trench.push(current_position);
    }

    trench
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/18/example-1.txt", "./inputs/18/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);
        let input = interpret_colors(&input);

        let trench = fast_dig(&input);

        println!("Trench:\n{:?}", trench);
        // let trench = dig_interior(&trench);

        // let capacity = trench.len();

        // println!("Capacity: {}", capacity);

        break;
    }

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

    use crate::task_18::interpret_colors;

    use super::{dig_interior, dig_trench, interior_to_string, parse_input, Position, Trench};

    #[test]
    fn dig_trench_from_example_should_match_trench1() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string("./inputs/18/example-1.txt")?;
        let input = parse_input(input);

        let actual = dig_trench(&input);
        let actual = interior_to_string(&actual);

        let expected = fs::read_to_string("./inputs/18/trench-1.txt")?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn dig_interior_from_example_should_match_trench2() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string("./inputs/18/example-1.txt")?;
        let input = parse_input(input);

        let trench = dig_trench(&input);
        let trench = dig_interior(&trench);

        let expected = fs::read_to_string("./inputs/18/trench-2.txt")?;
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

        assert_eq!(trench, expected);

        Ok(())
    }

    #[test]
    fn dig_interior_should_work_with_a_3_by_3() {
        let trench: Trench = HashSet::from([
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ]);

        let interior = dig_interior(&trench);
        let interior = interior_to_string(&interior);

        let expected = "###\n###\n###";

        assert_eq!(interior, expected);
    }

    #[test]
    fn interpret_colors_should_behave_as_in_example() -> Result<(), Box<dyn Error>> {
        let expected = fs::read_to_string("inputs/18/interpreted-1.txt")?;
        let expected = parse_input(expected);

        let actual = fs::read_to_string("./inputs/18/example-1.txt")?;
        let actual = interpret_colors(&parse_input(actual));

        assert_eq!(actual, expected);

        Ok(())
    }
}
