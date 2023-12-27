use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    fs,
};

type Position = (i32, i32);

fn add_positions(x: &Position, y: &Position) -> Position {
    (x.0 + y.0, x.1 + y.1)
}

fn negate_position(position: &Position) -> Position {
    (-position.0, -position.1)
}

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

fn max_position(field: &Field) -> Position {
    let max_x = *field.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let max_y = *field.keys().map(|(_, y)| y).max().unwrap_or(&0);

    (max_x, max_y)
}

/*
We want Dijkstra, and we take inspiration from
https://doc.rust-lang.org/std/collections/binary_heap/index.html
*/

#[derive(Debug, PartialEq, Eq)]
struct State {
    position: Position,
    direction: Position,
    velocity: i8,
    cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn initial_state(position: &Position) -> State {
    State {
        position: *position,
        direction: (0, 0),
        velocity: 0,
        cost: 0,
    }
}

fn get_next_states(field: &Field, state: &State) -> Vec<State> {
    let mut next_states: Vec<State> = Vec::new();

    if state.velocity < 3 && state.position != (0, 0) {
        let next_position = add_positions(&state.position, &state.direction);

        if let Some(next_cost) = field.get(&next_position) {
            next_states.push(State {
                position: next_position,
                direction: state.direction,
                velocity: state.velocity + 1,
                cost: state.cost + next_cost,
            });
        }
    }

    let side_directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .into_iter()
        .filter(|candidate| {
            candidate != &state.direction && candidate != &negate_position(&state.direction)
        });

    for side_direction in side_directions {
        let side_position = add_positions(&state.position, &side_direction);
        if let Some(next_cost) = field.get(&side_position) {
            next_states.push(State {
                position: side_position,
                direction: side_direction,
                velocity: 1,
                cost: state.cost + next_cost,
            });
        }
    }

    next_states
}

fn cheapest_path(field: &Field, from: &Position, to: &Position) -> Option<i32> {
    let mut position_to_cost: HashMap<Position, i32> = HashMap::from([(*from, 0)]);
    let mut heap = BinaryHeap::from([initial_state(from)]);

    while let Some(state) = heap.pop() {
        if &state.position == to {
            return Some(state.cost);
        }

        let next_states = get_next_states(field, &state);

        for next_state in next_states {
            //Skip next_state, if we know a cheaper path already
            if let Some(existing_cost) = position_to_cost.get(&next_state.position) {
                if existing_cost <= &next_state.cost {
                    continue;
                }
            }

            position_to_cost.insert(next_state.position, next_state.cost);
            heap.push(next_state);
        }
    }

    None
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/17/example-1.txt", "./inputs/17/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let field = parse_input(input);

        let target = max_position(&field);
        let cost = cheapest_path(&field, &(0, 0), &target).unwrap_or(-1);

        println!("Found cost: {}", cost);

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

#[cfg(test)]
mod tests {
    use super::{cheapest_path, get_next_states, initial_state, parse_input, State};

    #[test]
    fn get_next_states_should_find_initial_successors() {
        let field = parse_input("12\n34".to_string());
        let state = &initial_state(&(0, 0));

        let initial_successors = get_next_states(&field, state);

        let expected = Vec::from([
            State {
                position: (1, 0),
                direction: (1, 0),
                velocity: 1,
                cost: 2,
            },
            State {
                position: (0, 1),
                direction: (0, 1),
                velocity: 1,
                cost: 3,
            },
        ]);

        assert_eq!(initial_successors, expected);
    }

    #[test]
    fn should_correctly_compute_a_simple_cheapest_path() {
        let field = parse_input("12\n34".to_string());

        let expected = Some(6);
        let actual = cheapest_path(&field, &(0, 0), &(1, 1));

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_correctly_find_a_velocity_maxing_cheapest_path() {
        let field = parse_input(["111111", "222221"].join("\n"));

        /*
        Our field is this:
        111111
        222221
        We expect a route like this:
        1>>>111
        222v>>>
        We expect cost like this:
        1+1+1+2+2+1 = 8
        */

        let expected = Some(8);
        let actual = cheapest_path(&field, &(0, 0), &(5, 1));

        assert_eq!(actual, expected);
    }
}
