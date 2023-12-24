use std::{error::Error, fs, iter};

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

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/14/example-1.txt", "./inputs/14/input.txt"];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let contents = contents
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();

        println!("Read file: {}", path);
        println!("File contents:\n{}", contents.join("\n"));

        break;
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented.");

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

    use super::move_north;

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
        let input = input.lines().map(|line| line.to_string()).collect::<Vec<_>>();

        let actual = move_north(&input);

        assert_eq!(actual, expected);
    }
}
