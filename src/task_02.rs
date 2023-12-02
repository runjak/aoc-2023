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
    let game_parts = line[prefix.len()..].split(":").collect::<Vec<_>>();
    let game_id = game_parts.get(0)?.parse::<u32>().ok()?;
    let drawing_strings = game_parts.get(1)?.split("; ").collect::<Vec<_>>();

    // We parse the drawings:
    let drawings: Vec<Drawing> = drawing_strings
        .iter()
        .map(|drawing| -> Drawing {
            return drawing.split(", ").filter_map(|part| -> Option<(String, u32)> {
                let part_parts = part.split(" ").collect::<Vec<_>>();

                let color = part_parts.get(1)?.to_string();
                let count = part_parts.get(0)?.parse::<u32>().ok()?;

                Some((color, count))
            }).collect::<Drawing>();
        })
        .collect();

    Some((game_id, drawings))
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("02-1:");
    println!("Work in progress.");

    Ok(())
}
