use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

use ndarray::{Array, Ix1, Ix2};

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

type ConnectionMap = HashMap<Position, Vec<Position>>;

fn input_map_to_connection_map(input: &InputMap) -> ConnectionMap {
    input
        .keys()
        .map(|(x, y)| -> (Position, Vec<Position>) {
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

type Vector = Array<u32, Ix1>;
type Matrix = Array<u32, Ix2>;

struct MatrixContainer {
    index_to_position: HashMap<usize, Position>,
    position_to_index: HashMap<Position, usize>,
    matrix: Matrix,
}

fn connection_map_to_matrix(connection_map: &ConnectionMap) -> MatrixContainer {
    let index_to_position = connection_map
        .keys()
        .enumerate()
        .map(|(index, position)| -> (usize, Position) { (index, *position) })
        .collect::<HashMap<_, _>>();

    let position_to_index = index_to_position
        .iter()
        .map(|(index, position)| (*position, *index))
        .collect::<HashMap<_, _>>();

    let mut matrix: Matrix = Array::zeros((index_to_position.len(), index_to_position.len()));

    for (from_index, from_position) in index_to_position.iter() {
        let Some(to_positions) = connection_map.get(from_position) else {
            continue;
        };

        for to_position in to_positions {
            let Some(to_index) = position_to_index.get(to_position) else {
                continue;
            };

            matrix[(*from_index, *to_index)] = 1;
        }
    }

    MatrixContainer {
        index_to_position,
        position_to_index,
        matrix,
    }
}

fn reachable_in_steps(input: &InputMap, steps: usize) -> HashSet<Position> {
    let connection_map = input_map_to_connection_map(input);
    let matrix_container = connection_map_to_matrix(&connection_map);
    let mut matrix = matrix_container.matrix.clone();

    let mut steps = steps;
    while steps > 1 {
        matrix = matrix.dot(&matrix_container.matrix);
        steps -= 1;

        // if steps % 2 == 1 {
        //     matrix = matrix.dot(&matrix_container.matrix);
        //     steps -= 1;
        // } else {
        //     matrix = matrix.dot(&matrix);
        //     steps /= 2;
        // }
    }

    let Some(start_position) = input
        .iter()
        .filter(|(_, tile)| **tile == TileType::Start)
        .map(|(position, _)| *position)
        .next()
    else {
        return HashSet::new();
    };

    let Some(start_index) = matrix_container.position_to_index.get(&start_position) else {
        return HashSet::new();
    };

    let mut start_index_vector: Vector = Array::zeros(matrix_container.index_to_position.len());
    start_index_vector[*start_index] = 1;

    let reachable_index_vector: Vector = start_index_vector.dot(&matrix);

    reachable_index_vector
        .iter()
        .enumerate()
        .filter_map(|(reachable_index, value)| -> Option<Position> {
            if *value < 1 {
                return None;
            }

            matrix_container
                .index_to_position
                .get(&reachable_index)
                .copied()
        })
        .collect()
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
