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
type Trench = HashMap<Position, Color>;

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
    let mut lagoon: Trench = HashMap::from([(current_position, "".to_string())]);

    for dig in input {
        for _ in 0..dig.length {
            current_position = apply_direction(&current_position, &dig.direction);
            lagoon.insert(current_position, dig.color.to_string());
        }
    }

    lagoon
}

fn shift_positive(trench: &Trench) -> Trench {
    let min_x = trench.keys().map(|(x, _)| x).min().unwrap_or(&0);
    let min_y = trench.keys().map(|(_, y)| y).min().unwrap_or(&0);

    trench
        .iter()
        .map(|((x, y), c)| -> (Position, Color) { ((x - min_x, y - min_y), c.to_string()) })
        .collect()
}

type Interior = HashSet<Position>;

fn dig_interior(trench: &Trench) -> Interior {
    let trench = shift_positive(trench);

    let outside_x = *trench.keys().map(|(x, _)| x).max().unwrap_or(&0) + 1;
    let outside_y = *trench.keys().map(|(_, y)| y).max().unwrap_or(&0) + 1;

    let mut interior: Interior = (-1..=outside_x + 1)
        .flat_map(|x| -> Vec<Position> { (-1..=outside_y).map(|y| (x, y)).collect() })
        .collect();

    for trench_position in trench.keys() {
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

    for trench_position in trench.keys() {
        interior.insert(*trench_position);
    }

    interior
}

#[allow(dead_code)]
fn interior_to_string(interior: &Interior) -> String {
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
    use std::{
        collections::{HashMap, HashSet},
        error::Error,
        fs,
    };

    use super::{dig_interior, dig_trench, interior_to_string, parse_input, Position, Trench};

    #[test]
    fn dig_trench_from_example_should_match_trench1() -> Result<(), Box<dyn Error>> {
        let input = fs::read_to_string("./inputs/18/example-1.txt")?;
        let input = parse_input(input);

        let actual = dig_trench(&input)
            .keys()
            .map(|p| *p)
            .collect::<HashSet<_>>();
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
        let trench: Trench = HashMap::from([
            ((0, 0), "".to_string()),
            ((1, 0), "".to_string()),
            ((2, 0), "".to_string()),
            ((0, 1), "".to_string()),
            ((2, 1), "".to_string()),
            ((0, 2), "".to_string()),
            ((1, 2), "".to_string()),
            ((2, 2), "".to_string()),
        ]);

        let interior = dig_interior(&trench);
        let interior = interior_to_string(&interior);

        let expected = "###\n###\n###";

        assert_eq!(interior, expected);
    }
}
