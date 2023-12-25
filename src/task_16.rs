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

#[derive(Debug, Clone, Copy)]
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

fn crazy_diamond(field: &Field) -> HashSet<Position> {
    let mut energized: HashSet<Position> = HashSet::new();
    let mut lasers: Vec<Laser> = Vec::from([((0, 0), Direction::Right)]);

    while !lasers.is_empty() {
        let Some(laser) = lasers.pop() else {
            break;
        };

        energized.insert(laser.0);
        lasers.append(&mut shine_on(field, &laser));
    }

    energized
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/16/example-1.txt", "./inputs/16/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        let energized = crazy_diamond(&input);

        println!("Counting energized positions: {}", energized.len());

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
