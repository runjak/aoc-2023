use regex::Regex;
use std::{error::Error, fs};

#[derive(Debug)]
struct Card {
    id: u32,
    wins: Vec<u32>,
    gots: Vec<u32>,
}

fn parse_card(line: &str) -> Option<Card> {
    let card_regex = Regex::new(r"Card +(?<id>[0-9]+):(?<wins>[0-9 ]+)|(?<gots>[0-9 ]+)^$").ok()?;

    let captures = card_regex.captures(line)?;

    let id = captures.name("id")?.as_str().parse::<u32>().ok()?;

    let wins = captures.name("wins")?.as_str();
    let gots = captures.name("gots")?.as_str();

    let wins = wins
        .split(" ")
        .filter_map(|digits| digits.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let gots = gots
        .split(" ")
        .filter_map(|digits| digits.parse::<u32>().ok())
        .collect::<Vec<_>>();

    Some(Card { id, wins, gots })
}

pub fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/04/example-1.txt", "./inputs/04/input.txt"];

    for path in paths {
        println!("Reading file {}.", path);
        let contents = fs::read_to_string(path)?;

        println!("{}", contents);

        let cards = contents.lines().flat_map(parse_card).collect::<Vec<_>>();

        println!("Got cards: {:?}", cards);

        break;
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("04-1:");
    first()?;

    Ok(())
}
