use std::{collections::HashMap, error::Error};

type Drawing = HashMap<String, u32>;
type Game = (u32, Vec<Drawing>);
type GameLog = Vec<Game>;

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

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("02-1:");
    println!("Work in progress.");

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
            HashMap::from([(String::from("red"), 1), (String::from("green"), 2), (String::from("blue"), 6)]),
            HashMap::from([(String::from("green"), 2)]),
        ]);
        let expected: Option<Game> = Some((1, drawings));

        let game = parse_game(example);

        assert_eq!(game, expected);
    }
}
