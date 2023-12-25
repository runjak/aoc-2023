use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_in_direction((x, y): &Position, direction: &Direction) -> Position {
    match direction {
        Direction::Up => (*x, y - 1),
        Direction::Down => (*x, y + 1),
        Direction::Left => (x - 1, *y),
        Direction::Right => (x + 1, *y),
    }
}

fn affect_direction(symbol: &char, direction: &Direction) -> Vec<Direction> {
    match (symbol, direction) {
        ('/', Direction::Up) => Vec::from([Direction::Right]),
        ('/', Direction::Down) => Vec::from([Direction::Left]),
        ('/', Direction::Left) => Vec::from([Direction::Down]),
        ('/', Direction::Right) => Vec::from([Direction::Up]),
        ('\\', Direction::Up) => Vec::from([Direction::Left]),
        ('\\', Direction::Down) => Vec::from([Direction::Right]),
        ('\\', Direction::Left) => Vec::from([Direction::Up]),
        ('\\', Direction::Right) => Vec::from([Direction::Down]),
        ('-', Direction::Up) => Vec::from([Direction::Left, Direction::Right]),
        ('-', Direction::Down) => Vec::from([Direction::Left, Direction::Right]),
        ('-', Direction::Left) => Vec::from([Direction::Left]),
        ('-', Direction::Right) => Vec::from([Direction::Right]),
        ('|', Direction::Up) => Vec::from([Direction::Up]),
        ('|', Direction::Down) => Vec::from([Direction::Down]),
        ('|', Direction::Left) => Vec::from([Direction::Up, Direction::Down]),
        ('|', Direction::Right) => Vec::from([Direction::Up, Direction::Down]),
        (_, _) => Vec::from([*direction]),
    }
}

type Laser = (Position, Direction);

fn shine_on(field: &Field, laser: &Laser) -> Vec<Laser> {
    let (position, direction) = laser;

    let position = move_in_direction(position, direction);
    let Some(symbol) = field.get(&position) else {
        return Vec::new();
    };

    affect_direction(symbol, direction)
        .iter()
        .map(|d| (position, *d))
        .collect()
}

fn crazy_diamond(field: &Field, initial_laser: &Laser) -> HashSet<Position> {
    let mut visited: HashSet<Laser> = HashSet::from([]);
    let mut lasers: Vec<Laser> = Vec::from([*initial_laser]);

    while !lasers.is_empty() {
        let Some(laser) = lasers.pop() else {
            break;
        };

        for next_laser in shine_on(field, &laser) {
            if !visited.contains(&next_laser) {
                lasers.push(next_laser);
                visited.insert(next_laser);
            }
        }
    }

    visited.iter().map(|(p, _)| -> Position { *p }).collect()
}

#[allow(dead_code)]
fn print_energized(field: &Field, energized: &HashSet<Position>) {
    let max_x = *field.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let max_y = *field.keys().map(|(_, y)| y).max().unwrap_or(&0);

    let lines = (0..=max_y)
        .map(|y| -> String {
            (0..=max_x)
                .map(|x| -> char {
                    let position = &(x, y);

                    let x = *field.get(position).unwrap_or(&'.');

                    if x != '.' {
                        return x;
                    }

                    if energized.contains(position) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect::<Vec<_>>();

    println!("{}", lines.join("\n"));
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/16/example-1.txt", "./inputs/16/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = &parse_input(input);

        let initial_laser: Laser = ((-1, 0), Direction::Right);
        let energized = crazy_diamond(input, &initial_laser);

        println!("Energized positions: {}", energized.len());
    }

    Ok(())
}

fn initial_lasers(field: &Field) -> Vec<Laser> {
    let max_x = *field.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let max_y = *field.keys().map(|(_, y)| y).max().unwrap_or(&0);

    let top_lasers = (0..=max_x).map(|x| -> Laser { ((x, -1), Direction::Down) });
    let bottom_lasers = (0..=max_x).map(|x| -> Laser { ((x, max_y + 1), Direction::Up) });
    let left_lasers = (0..=max_y).map(|y| -> Laser { ((-1, y), Direction::Right) });
    let right_lasers = (0..=max_y).map(|y| -> Laser { ((max_x + 1, y), Direction::Left) });

    top_lasers
        .chain(bottom_lasers)
        .chain(left_lasers)
        .chain(right_lasers)
        .collect()
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/16/example-1.txt", "./inputs/16/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = &parse_input(input);

        let max_energized = initial_lasers(&input)
            .iter()
            .map(|laser| crazy_diamond(&input, laser).len())
            .max()
            .unwrap_or(0);

        println!("Maximum energized positions: {}", max_energized);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("16-1:");
    first()?;
    println!("16-2:");
    second()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs};

    use super::{crazy_diamond, parse_input, Direction, Laser};

    #[test]
    fn crazy_diamond_should_have_energized_match_example() {
        let expected = fs::read_to_string("./inputs/16/expected-energized.txt").unwrap();
        let expected = parse_input(expected);
        let expected = expected
            .iter()
            .filter(|(_, c)| **c == '#')
            .map(|(p, _)| *p)
            .collect::<HashSet<_>>();

        let input = fs::read_to_string("./inputs/16/example-1.txt").unwrap();
        let input = parse_input(input);

        let initial_laser: Laser = ((-1, 0), Direction::Right);
        let actual = crazy_diamond(&input, &initial_laser);

        assert_eq!(actual, expected);
    }
}
