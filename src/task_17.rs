use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    fs, string,
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

fn cheapest_path(field: &Field, from: &Position, to: &Position) -> Option<State> {
    let mut position_to_cost: HashMap<Position, i32> = HashMap::from([(*from, 0)]);
    let mut heap = BinaryHeap::from([initial_state(from)]);

    while let Some(state) = heap.pop() {
        if &state.position == to {
            return Some(state);
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
        let cheapest_state = cheapest_path(&field, &(0, 0), &target);
        let cost = cheapest_state.map(|s| s.cost).unwrap_or(-1);

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

fn trace_cheapest_path(field: &Field, from: &Position, to: &Position) -> Vec<State> {
    let mut trace = Vec::new();

    let mut to: Position = *to;
    while &to != from {
        let Some(state) = cheapest_path(field, from, &to) else {
            break;
        };

        to = add_positions(&to, &negate_position(&state.direction));

        trace.push(state);
    }

    trace
}

fn stringify_field_and_states(field: &Field, states: &Vec<State>) -> String {
    let (max_x, max_y) = max_position(field);
    let mut field = field
        .into_iter()
        .map(|(position, cost)| -> (Position, String) { (*position, cost.to_string()) })
        .collect::<HashMap<_, _>>();

    for state in states {
        let s = match state.direction {
            (1, 0) => ">",
            (0, 1) => "v",
            (-1, 0) => "<",
            (0, -1) => "^",
            (_, _) => ".",
        }
        .to_string();

        field.insert(state.position, s);
    }

    let mut lines: Vec<String> = Vec::new();
    for y in 0..=max_y {
        let line = (0..=max_x)
            .map(|x| -> String { field.get(&(x, y)).unwrap_or(&"?".to_string()).to_string() })
            .collect::<Vec<_>>();

        lines.push(line.concat());
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use super::{
        cheapest_path, get_next_states, initial_state, max_position, parse_input,
        stringify_field_and_states, trace_cheapest_path, State,
    };

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
        let actual = cheapest_path(&field, &(0, 0), &(1, 1)).map(|s| s.cost);

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
        let actual = cheapest_path(&field, &(0, 0), &(5, 1)).map(|s| s.cost);

        assert_eq!(actual, expected);
    }

    #[test]
    fn cheapest_path_should_find_same_path_as_example() -> Result<(), Box<dyn Error>> {
        let example = fs::read_to_string("./inputs/17/example-1.txt")?;
        let example = parse_input(example);

        let expected = fs::read_to_string("./inputs/17/trace-1.txt")?;

        let to = &max_position(&example);
        let trace = trace_cheapest_path(&example, &(0, 0), to);
        let actual = stringify_field_and_states(&example, &trace);

        println!("Discovered the following path:\n{}", actual);

        assert_eq!(actual, expected);

        Ok(())
    }
}
