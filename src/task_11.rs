use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

type N = i128;
type Coordinate = (N, N);
type InputMap = HashMap<Coordinate, char>;

fn parse_input(contents: String) -> InputMap {
    contents
        .lines()
        .enumerate()
        .flat_map(|(row_index, line)| {
            let y = N::try_from(row_index).unwrap();

            line.chars().enumerate().map(move |(col_index, char)| {
                let x = N::try_from(col_index).unwrap();

                ((x, y), char)
            })
        })
        .collect::<InputMap>()
}

fn find_galaxies(input: &InputMap) -> Vec<Coordinate> {
    input
        .iter()
        .filter(|(_, c)| **c == '#')
        .map(|((x, y), _)| -> Coordinate { (*x, *y) })
        .collect::<Vec<_>>()
}

fn find_dimensions(galaxies: &Vec<Coordinate>) -> (N, N) {
    let x = galaxies.iter().map(|(x, _)| x).max().unwrap_or(&0);
    let y = galaxies.iter().map(|(_, y)| y).max().unwrap_or(&0);

    (*x, *y)
}

fn expand_galaxies(galaxies: &Vec<Coordinate>, additional_distance: N) -> Vec<Coordinate> {
    let galaxy_cols = galaxies.iter().map(|(x, _)| *x).collect::<HashSet<_>>();
    let galaxy_rows = galaxies.iter().map(|(_, y)| *y).collect::<HashSet<_>>();

    let (x_max, y_max) = find_dimensions(galaxies);

    let expand_cols = (0..x_max)
        .filter(|x| !galaxy_cols.contains(x))
        .collect::<HashSet<_>>();
    let expand_rows = (0..y_max)
        .filter(|y| !galaxy_rows.contains(y))
        .collect::<HashSet<_>>();

    galaxies
        .iter()
        .map(|(x, y)| -> Coordinate {
            let expand_x = expand_cols.iter().filter(|col| *col < x).count();
            let expand_y = expand_rows.iter().filter(|row| *row < y).count();

            let expand_x = N::try_from(expand_x).unwrap_or(0);
            let expand_y = N::try_from(expand_y).unwrap_or(0);

            (
                x + expand_x * additional_distance,
                y + expand_y * additional_distance,
            )
        })
        .collect()
}

fn distance(a: &Coordinate, b: &Coordinate) -> N {
    let (x1, y1) = a;
    let (x2, y2) = b;

    (x1 - x2).abs() + (y1 - y2).abs()
}

fn distances(galaxies: &Vec<Coordinate>) -> Vec<N> {
    let mut galaxies = galaxies.as_slice();
    let mut distances: Vec<N> = Vec::new();

    while galaxies.len() > 1 {
        let first = galaxies.first().unwrap();
        let others = &galaxies[1..];

        for other in others {
            distances.push(distance(first, other));
        }

        galaxies = others;
    }

    return distances;
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/11/example-1.txt", "./inputs/11/input.txt"];

    for path in paths {
        println!("Handling file {}:", path);

        let contents = fs::read_to_string(path)?;
        let input = parse_input(contents);
        let galaxies = find_galaxies(&input);
        let galaxies = expand_galaxies(&galaxies, 1);

        let sum = distances(&galaxies).iter().sum::<N>();
        println!("Sum of distances is {}", sum);
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/11/example-1.txt", "./inputs/11/input.txt"];

    for path in paths {
        println!("Handling file {}:", path);

        let contents = fs::read_to_string(path)?;
        let input = parse_input(contents);
        let galaxies = find_galaxies(&input);
        let galaxies = expand_galaxies(&galaxies, 1000000 - 1);

        let sum = distances(&galaxies).iter().sum::<N>();
        println!("Sum of distances is {}", sum);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("11-1:");
    first()?;
    println!("11-2:");
    second()?;

    Ok(())
}
