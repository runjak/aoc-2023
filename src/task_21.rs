use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    io::{stdout, Write},
};

#[derive(Debug, PartialEq, PartialOrd)]
enum TileType {
    Start,
    Plot,
    Rock,
    Reach,
}

type Position = (i32, i32);
type InputMap = HashMap<Position, TileType>;

fn parse_input_map(input: String) -> InputMap {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| -> Vec<(Position, TileType)> {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| -> Option<(Position, TileType)> {
                    let (x, y) = (i32::try_from(x).ok()?, i32::try_from(y).ok()?);

                    match c {
                        'S' => Some(((x, y), TileType::Start)),
                        '.' => Some(((x, y), TileType::Plot)),
                        '#' => Some(((x, y), TileType::Rock)),
                        'O' => Some(((x, y), TileType::Reach)),
                        _ => None,
                    }
                })
                .collect()
        })
        .collect()
}

type ConnectionMap = HashMap<Position, HashSet<Position>>;

fn input_map_to_connection_map(input: &InputMap) -> ConnectionMap {
    input
        .keys()
        .map(|(x, y)| -> (Position, HashSet<Position>) {
            let position = (*x, *y);

            let adjacent = [(x + 1, *y), (x - 1, *y), (*x, y + 1), (*x, y - 1)];

            (
                position,
                adjacent
                    .into_iter()
                    .filter(|adjacent| -> bool {
                        let Some(tile) = input.get(&adjacent) else {
                            return false;
                        };

                        *tile != TileType::Rock
                    })
                    .collect(),
            )
        })
        .collect()
}

fn combine_connection_maps(a: &ConnectionMap, b: &ConnectionMap) -> ConnectionMap {
    a.iter()
        .map(|(from, to_bs)| -> (Position, HashSet<Position>) {
            let from = *from;
            let to = to_bs
                .iter()
                .flat_map(|to| -> HashSet<Position> {
                    let Some(targets) = b.get(to) else {
                        return HashSet::new();
                    };

                    targets.clone()
                })
                .collect::<HashSet<_>>();

            (from, to)
        })
        .collect()
}

fn reachable_in_steps(input: &InputMap, steps: usize) -> HashSet<Position> {
    let connection_map = input_map_to_connection_map(input);
    let mut transition_map = connection_map.clone();

    let mut steps = steps;
    let mut shifts: Vec<bool> = Vec::new();
    while steps > 1 {
        if steps % 2 == 1 {
            shifts.push(false);
            steps -= 1;
        } else {
            shifts.push(true);
            steps /= 2;
        }
    }

    while let Some(do_shift) = shifts.pop() {
        print!(".");
        let _ = stdout().flush();
        if do_shift {
            transition_map = combine_connection_maps(&transition_map, &transition_map);
        } else {
            transition_map = combine_connection_maps(&transition_map, &connection_map);
        }
    }

    let Some(start_position) = input
        .iter()
        .filter(|(_, tile)| **tile == TileType::Start)
        .map(|(position, _)| *position)
        .next()
    else {
        return HashSet::new();
    };

    let Some(reachable) = transition_map.get(&start_position) else {
        return HashSet::new();
    };

    reachable.clone()
}

fn first() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./inputs/21/input.txt")?;
    let input = parse_input_map(input);

    let reachable = reachable_in_steps(&input, 64);

    println!("Number of reachable places: {}", reachable.len());

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("21-1:");
    first()?;
    println!("21-2:");
    second()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, error::Error, fs};

    use super::{parse_input_map, reachable_in_steps, Position, TileType};

    #[test]
    fn reachable_in_steps_should_behave_as_in_examples() -> Result<(), Box<dyn Error>> {
        let expecteds_paths = [
            "inputs/21/expected-1-1.txt",
            "inputs/21/expected-1-2.txt",
            "inputs/21/expected-1-3.txt",
            "inputs/21/expected-1-4.txt",
        ];
        let steps = [1_usize, 2, 3, 6];

        let mut steps_and_reachables: Vec<(usize, HashSet<Position>)> = Vec::new();

        for (steps, path) in steps.iter().zip(expecteds_paths.iter()) {
            let expected = fs::read_to_string(path)?;
            let expected = parse_input_map(expected);
            let expected = expected
                .iter()
                .filter(|(_, tile)| **tile == TileType::Reach)
                .map(|(position, _)| *position)
                .collect::<HashSet<_>>();

            steps_and_reachables.push((*steps, expected));
        }

        let input = fs::read_to_string("./inputs/21/example-1.txt")?;
        let input = parse_input_map(input);

        for (steps, expected) in steps_and_reachables {
            let reachables = reachable_in_steps(&input, steps);

            assert_eq!(
                reachables, expected,
                "Example should reach these after {} steps.",
                steps
            );
        }

        Ok(())
    }
}
