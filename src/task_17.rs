use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

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

fn heuristic_cost(from: &Position, to: &Position) -> i32 {
    /*
    The heuristic must be admissible as per [1].
    That is we need to never over-estimate the true cost.
    A simple way to achieve this is to assume minimal (1) cost per field.
    To get there we can just get away with calculating the manhattan distance.
    [1]: https://en.wikipedia.org/wiki/A*_search_algorithm
    */
    (to.0 - from.0).abs() + (to.1 - from.1).abs()
}

type Path = Vec<Position>;

fn is_path_done(path: &Path, target: &Position) -> bool {
    let Some(last) = path.last() else {
        return false;
    };

    last == target
}

fn next_positions((x, y): &Position, (max_x, max_y): &Position) -> Vec<Position> {
    [(x - 1, *y), (x + 1, *y), (*x, y - 1), (*x, y + 1)]
        .into_iter()
        .filter(|(x, y)| x >= &0 && y >= &0 && x <= max_x && y <= max_y)
        .collect()
}

fn is_path_end_valid(path: &Path) -> bool {
    /*
    Check that the last 4 positions of the path are not in a line.
    Also check that the path doesn't loop back on it's end.
    */
    if path.len() >= 3 {
        let last_three = path[path.len() - 3..].iter().collect::<HashSet<_>>();

        if last_three.len() < 3 {
            return false;
        }
    }

    if path.len() >= 4 {
        let last_four = path[path.len() - 4..].to_vec();
        let different_x = last_four
            .iter()
            .map(|(x, _)| *x)
            .collect::<HashSet<_>>()
            .len();
        let different_y = last_four
            .iter()
            .map(|(_, y)| *y)
            .collect::<HashSet<_>>()
            .len();

        if different_x == 1 || different_y == 1 {
            // Forbidden length of a segment detected.
            return false;
        }
    }

    true
}

type Cost = i32;

fn next_paths_and_costs(
    field: &Field,
    max_position: &Position,
    path: &Path,
    cost: Cost,
) -> Vec<(Path, Cost)> {
    let Some(last_position) = path.last() else {
        return Vec::new();
    };

    next_positions(last_position, max_position)
        .into_iter()
        .filter_map(|next_position| -> Option<(Path, Cost)> {
            let mut next_path = path.clone();
            next_path.push(next_position);

            if !is_path_end_valid(&next_path) {
                return None;
            }

            let next_cost = cost + *field.get(&next_position).unwrap_or(&0);

            Some((next_path, next_cost))
        })
        .collect()
}

fn find_path(field: &Field, from: &Position, to: &Position) -> Option<(Path, Cost)> {
    let mut paths_and_costs: Vec<(Path, Cost)> = Vec::from([(Vec::from([*from]), 0)]);

    while !paths_and_costs.is_empty() {
        // Sort paths_and_costs to have best candidate at the end
        paths_and_costs.sort_by_key(|(path, cost)| -> i32 {
            let path_end = path.last().unwrap(); // Assume non-empty paths

            // Returning negative value, to have best candidate at the ned.
            -(heuristic_cost(path_end, to) + *cost)
        });

        // Assume existing path and cost as per !..is_empty()
        let (path, cost) = paths_and_costs.pop().unwrap();

        // Check if best current path is the solution
        if is_path_done(&path, to) {
            return Some((path.clone(), cost));
        }

        // Find next paths
        paths_and_costs.append(&mut next_paths_and_costs(field, to, &path, cost));
    }

    None
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/17/example-1.txt", "./inputs/17/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let field = parse_input(input);

        let target = get_max_position(&field);
        let foo = find_path(&field, &(0, 0), &target);

        println!("Got something cool:\n{:?}", foo);

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
