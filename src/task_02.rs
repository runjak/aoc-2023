use std::{collections::HashMap, error::Error, fs};

type Drawing = HashMap<String, u32>;
type Game = (u32, Vec<Drawing>);

fn parse_game(line: &str) -> Option<Game> {
    let prefix = "Game ";

    if !line.starts_with(prefix) {
        return None;
    }

    // We identify the id and drawing parts of a game:
    let game_parts = line[prefix.len()..].split(": ").collect::<Vec<_>>();
    let game_id = game_parts.get(0)?.parse::<u32>().ok()?;
    let drawing_strings = game_parts.get(1)?.split("; ").collect::<Vec<_>>();

    // We parse the drawings:
    let drawings: Vec<Drawing> = drawing_strings
        .iter()
        .map(|drawing| -> Drawing {
            return drawing
                .split(", ")
                .filter_map(|part| -> Option<(String, u32)> {
                    let part_parts = part.split(" ").collect::<Vec<_>>();

                    let color = part_parts.get(1)?.to_string();
                    let count = part_parts.get(0)?.parse::<u32>().ok()?;

                    Some((color, count))
                })
                .collect::<Drawing>();
        })
        .collect();

    Some((game_id, drawings))
}

fn game_was_possible(drawings: &Vec<Drawing>, limits: &Drawing) -> bool {
    return drawings.iter().all(|drawing| -> bool {
        return drawing.iter().all(|(color, count)| -> bool {
            return count <= limits.get(color).unwrap_or(&0);
        });
    });
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/02/example-1.txt", "./inputs/02/input.txt"];

    for path in paths {
        println!("Reading file {}", path);
        let file = fs::read_to_string(path)?;

        let games: Vec<_> = file.lines().filter_map(parse_game).collect();

        // 12 red cubes, 13 green cubes, and 14
        let limits: Drawing = HashMap::from([
            (String::from("red"), 12),
            (String::from("green"), 13),
            (String::from("blue"), 14),
        ]);

        let sum: u32 = games
            .iter()
            .filter(|(_, drawings)| game_was_possible(drawings, &limits))
            .map(|(game_id, _)| game_id)
            .sum();

        println!("Sum is {}", sum);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("02-1:");
    first()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::task_02::{parse_game, Drawing, Game};

    #[test]
    fn parse_game_parses_first_example_line() {
        let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let drawings: Vec<Drawing> = Vec::from([
            HashMap::from([(String::from("blue"), 3), (String::from("red"), 4)]),
            HashMap::from([
                (String::from("red"), 1),
                (String::from("green"), 2),
                (String::from("blue"), 6),
            ]),
            HashMap::from([(String::from("green"), 2)]),
        ]);
        let expected: Option<Game> = Some((1, drawings));

        let game = parse_game(example);

        assert_eq!(game, expected);
    }
}
