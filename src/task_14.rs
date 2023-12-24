use std::{collections::HashMap, error::Error, fs, iter};

static FIXED_ROCK: char = '#';
static MOVABLE_ROCK: char = 'O';
static EMPTY_SPACE: char = '.';

fn move_west(lines: &Vec<String>) -> Vec<String> {
    lines
        .iter()
        .map(|line| -> String {
            line.split(FIXED_ROCK)
                .map(|chunk| -> String {
                    let movable_count = chunk.chars().filter(|c| *c == MOVABLE_ROCK).count();
                    let space_count = chunk.len() - movable_count;

                    let rocks = iter::repeat(MOVABLE_ROCK).take(movable_count);
                    let spaces = iter::repeat(EMPTY_SPACE).take(space_count);

                    rocks.chain(spaces).collect()
                })
                .collect::<Vec<_>>()
                .join(&FIXED_ROCK.to_string())
        })
        .collect()
}

fn transpose_lines(lines: &Vec<String>) -> Vec<String> {
    // Inspired by https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust

    if lines.is_empty() {
        return Vec::new();
    }

    let width = lines[0].len();
    let mut iters = lines
        .into_iter()
        .map(|line| line.chars())
        .collect::<Vec<_>>();

    (0..width)
        .map(|_| {
            iters
                .iter_mut()
                .filter_map(|i| i.next())
                .collect::<String>()
        })
        .collect()
}

fn move_north(lines: &Vec<String>) -> Vec<String> {
    transpose_lines(&move_west(&transpose_lines(lines)))
}

type N = usize;

fn compute_load(lines: &Vec<String>) -> N {
    let max_load = lines.len();

    lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let line_load = max_load - index;
            let rock_count = line.chars().filter(|c| *c == MOVABLE_ROCK).count();

            line_load * rock_count
        })
        .sum()
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/14/example-1.txt", "./inputs/14/input.txt"];

    for path in paths {
        println!("Handling file: {}", path);

        let contents = fs::read_to_string(path)?;
        let contents = contents
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();

        let moved = move_north(&contents);
        let load = compute_load(&moved);

        println!("Computed load: {}", load);
    }

    Ok(())
}

fn reverse_lines(lines: &Vec<String>) -> Vec<String> {
    lines
        .iter()
        .map(|line| line.chars().rev().collect())
        .collect()
}

fn move_south(lines: &Vec<String>) -> Vec<String> {
    transpose_lines(&reverse_lines(&move_west(&reverse_lines(
        &transpose_lines(lines),
    ))))
}

fn move_east(lines: &Vec<String>) -> Vec<String> {
    reverse_lines(&move_west(&reverse_lines(lines)))
}

fn spin_cycle(lines: &Vec<String>) -> Vec<String> {
    move_east(&move_south(&move_west(&move_north(lines))))
}

fn spin_cycles(lines: &Vec<String>) -> Vec<String> {
    let mut lines = lines.clone();

    type Move = u32;
    let total_moves: Move = 1_000_000_000;

    let mut inputs_at_moves: HashMap<String, Move> = HashMap::from([(lines.join("\n"), 0)]);
    let mut remaining_moves: Move = 0;

    for moves_so_far in 1..=total_moves {
        let next_lines = spin_cycle(&lines);
        let key = next_lines.join("\n");

        if inputs_at_moves.contains_key(&key) {
            // Time travel as far as we can.
            remaining_moves = total_moves - moves_so_far;
            let cycle_length = moves_so_far - *inputs_at_moves.get(&key).unwrap();
            remaining_moves %= cycle_length;
            break;
        } else {
            inputs_at_moves.insert(key, moves_so_far);
        }

        lines = next_lines;
    }

    for _ in 0..=remaining_moves {
        lines = spin_cycle(&lines);
    }

    lines
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/14/example-1.txt", "./inputs/14/input.txt"];

    for path in paths {
        println!("Handling file: {}", path);

        let contents = fs::read_to_string(path)?;
        let contents = contents
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();

        let moved = spin_cycles(&contents);
        let load = compute_load(&moved);

        println!("Computed load: {}", load);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("14-1:");
    first()?;
    println!("14-2:");
    second()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{move_north, spin_cycle};

    #[test]
    fn move_north_should_behave_as_in_example() {
        let expected = [
            "OOOO.#.O..".to_string(),
            "OO..#....#".to_string(),
            "OO..O##..O".to_string(),
            "O..#.OO...".to_string(),
            "........#.".to_string(),
            "..#....#.#".to_string(),
            "..O..#.O.O".to_string(),
            "..O.......".to_string(),
            "#....###..".to_string(),
            "#....#....".to_string(),
        ]
        .to_vec();

        let input = fs::read_to_string("./inputs/14/example-1.txt").unwrap();
        let input = input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();

        let actual = move_north(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn spin_cycle_should_behave_as_in_example() {
        let [expected_1, expected_2, expected_3] = [
            [
                ".....#....".to_string(),
                "....#...O#".to_string(),
                "...OO##...".to_string(),
                ".OO#......".to_string(),
                ".....OOO#.".to_string(),
                ".O#...O#.#".to_string(),
                "....O#....".to_string(),
                "......OOOO".to_string(),
                "#...O###..".to_string(),
                "#..OO#....".to_string(),
            ]
            .to_vec(),
            [
                ".....#....".to_string(),
                "....#...O#".to_string(),
                ".....##...".to_string(),
                "..O#......".to_string(),
                ".....OOO#.".to_string(),
                ".O#...O#.#".to_string(),
                "....O#...O".to_string(),
                ".......OOO".to_string(),
                "#..OO###..".to_string(),
                "#.OOO#...O".to_string(),
            ]
            .to_vec(),
            [
                ".....#....".to_string(),
                "....#...O#".to_string(),
                ".....##...".to_string(),
                "..O#......".to_string(),
                ".....OOO#.".to_string(),
                ".O#...O#.#".to_string(),
                "....O#...O".to_string(),
                ".......OOO".to_string(),
                "#...O###.O".to_string(),
                "#.OOO#...O".to_string(),
            ]
            .to_vec(),
        ];

        let input = fs::read_to_string("./inputs/14/example-1.txt").unwrap();
        let input = input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();

        let [actual_1, actual_2, actual_3] = [
            spin_cycle(&input),
            spin_cycle(&spin_cycle(&input)),
            spin_cycle(&spin_cycle(&spin_cycle(&input))),
        ];

        assert_eq!(actual_1, expected_1);
        assert_eq!(actual_2, expected_2);
        assert_eq!(actual_3, expected_3);
    }
}
