use std::{collections::HashMap, error::Error, fs};

type Coordinate = (i32, i32);
type Input = HashMap<Coordinate, char>;

fn parse_input(input: String) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| -> Vec<(Coordinate, char)> {
            line.chars()
                .enumerate()
                .map(|(x, char)| -> (Coordinate, char) {
                    ((x.try_into().unwrap(), y.try_into().unwrap()), char)
                })
                .collect()
        })
        .collect()
}

fn find_start(input: &Input) -> Coordinate {
    for (coordinate, symbol) in input.iter() {
        if 'S' == *symbol {
            return coordinate.to_owned();
        }
    }

    todo!("Unreachable by problem definition.");
}

// One sided transition possibility, may return invalid coordinates.
fn reachable(input: &Input, from: &Coordinate) -> Vec<Coordinate> {
    let symbol = input.get(from).unwrap_or(&'.');
    let (x, y) = from.to_owned();

    match symbol {
        '|' => Vec::from([(x, y - 1), (x, y + 1)]),
        '-' => Vec::from([(x - 1, y), (x + 1, y)]),
        'L' => Vec::from([(x, y - 1), (x + 1, y)]),
        'J' => Vec::from([(x, y - 1), (x - 1, y)]),
        '7' => Vec::from([(x, y + 1), (x - 1, y)]),
        'F' => Vec::from([(x, y + 1), (x + 1, y)]),
        '.' => Vec::new(),
        'S' => Vec::from([(x, y - 1), (x - 1, y), (x, y + 1), (x + 1, y)]),
        _ => Vec::new(),
    }
}

// Two-sided transition possibility
fn connected(input: &Input, from: &Coordinate) -> Vec<Coordinate> {
    reachable(input, from)
        .iter()
        .filter(|next| {
            let next_nexts = reachable(input, next);

            next_nexts.iter().any(|next_next| next_next == from)
        })
        .map(|c| c.to_owned())
        .collect()
}

type StepsTo = HashMap<Coordinate, i32>;

fn flood_fill(input: &Input, from: &Coordinate) -> StepsTo {
    let mut steps_to = HashMap::from([(from.to_owned(), 0)]);
    let mut nexts = connected(input, from);
    let mut steps_so_far = 1;

    while !&nexts.is_empty() {
        let mut over_nexts: Vec<Coordinate> = Vec::new();

        for next in nexts {
            if !steps_to.contains_key(&next) {
                steps_to.insert(next.to_owned(), steps_so_far);
                over_nexts.append(&mut connected(input, &next));
            }
        }
        steps_so_far += 1;

        // We could consider filtering the over_nexts here to not contain stuff already in steps_to
        nexts = over_nexts;
    }

    steps_to
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = [
        "./inputs/10/example-1.txt",
        "./inputs/10/example-2.txt",
        "./inputs/10/input.txt",
    ];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        let start = find_start(&input);
        let steps_to = flood_fill(&input, &start);

        let max_steps = steps_to.values().max().unwrap();
        println!("Max steps are {}", max_steps);
    }

    Ok(())
}

fn replace_start(input: &Input) -> Input {
    let start = find_start(input);
    let nexts = connected(input, &start);

    let different_ys = nexts.iter().filter(|n| n.1 != start.1).count();
    let has_x_right = nexts.iter().any(|n| n.0 > start.0);
    let has_y_bottom = nexts.iter().any(|n| n.1 > start.1);

    let replacement = if different_ys >= 2 {
        '|'
    } else if different_ys > 0 {
        if has_x_right {
            if has_y_bottom {
                'F'
            } else {
                'L'
            }
        } else {
            '.'
        }
    } else {
        '.'
    };

    input
        .iter()
        .map(|(coordinates, symbol)| -> (Coordinate, char) {
            (
                *coordinates,
                if symbol == &'S' { replacement } else { *symbol },
            )
        })
        .collect()
}

fn count_insides(input: &Input, steps_to: &StepsTo) -> i32 {
    let input = replace_start(input);
    let x_max = *input.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let y_max = *input.keys().map(|(_, y)| y).max().unwrap_or(&0);

    let mut inside_count = 0;

    for y in 0..y_max {
        let mut is_outside = true;

        for x in 0..x_max {
            let coordinate = (x, y);
            let on_cycle = steps_to.contains_key(&coordinate);
            let symbol = *input.get(&coordinate).unwrap_or(&'.');

            if on_cycle {
                if symbol == '|' {
                    is_outside = !is_outside;
                } else if symbol == 'F' || symbol == 'L' {
                    let first_closing_edge = (x + 1..x_max)
                        .filter_map(|x_next| -> Option<char> {
                            // Return an Option of a closing edge :)
                            let next_coordinate = (x_next, y);

                            if !steps_to.contains_key(&next_coordinate) {
                                return None;
                            }

                            let next_symbol = *input.get(&next_coordinate).unwrap_or(&'.');

                            if next_symbol == '7' || next_symbol == 'J' {
                                return Some(next_symbol);
                            }

                            None
                        })
                        .next()
                        .unwrap_or('.');

                    match (symbol, first_closing_edge) {
                        ('F', 'J') | ('L', '7') => {
                            is_outside = !is_outside;
                        }
                        _ => {}
                    }
                }
            } else if !is_outside {
                // !on_cycle
                inside_count += 1;
            }
        }
    }

    inside_count
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented.");
    let paths = [
        "./inputs/10/example-3.txt",
        "./inputs/10/example-4.txt",
        "./inputs/10/example-5.txt",
        "./inputs/10/input.txt",
    ];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let input = parse_input(input);

        let start = find_start(&input);
        let steps_to = flood_fill(&input, &start);

        let insides = count_insides(&input, &steps_to);

        println!("Insides counted are {}", insides);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("10-1:");
    first()?;
    println!("10-2:");
    second()?;

    Ok(())
}
