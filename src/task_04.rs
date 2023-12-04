use regex::Regex;
use std::{error::Error, fs};

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    wins: Vec<u32>,
    gots: Vec<u32>,
}

fn parse_card(line: &str) -> Option<Card> {
    let card_regex = Regex::new(r"Card\s+(?<id>\d+):(?<wins>[\d\s]+)\|(?<gots>[\d\s]+)").ok()?;

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

fn count_matching_values(card: &Card) -> u32 {
    card.gots
        .iter()
        .filter(|got| card.wins.iter().any(|win| win == *got))
        .count()
        .try_into()
        .unwrap_or(0)
}

fn card_value(card: &Card) -> u32 {
    let winning_count = count_matching_values(card);

    if winning_count == 0 {
        return 0;
    }

    2_u32.pow(winning_count - 1)
}

pub fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/04/example-1.txt", "./inputs/04/input.txt"];

    for path in paths {
        println!("Reading file {}.", path);
        let contents = fs::read_to_string(path)?;

        let cards = contents.lines().flat_map(parse_card);

        let sum = cards.map(|card| card_value(&card)).sum::<u32>();

        println!("Sum of card values: {}", sum);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("04-1:");
    first()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{card_value, parse_card, Card};

    #[test]
    fn parse_card_should_parse_the_first_example() {
        let example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let expected = Some(Card {
            id: 1,
            wins: [41, 48, 83, 86, 17].to_vec(),
            gots: [83, 86, 6, 31, 17, 9, 48, 53].to_vec(),
        });

        let actual = parse_card(example);

        assert_eq!(actual, expected)
    }

    #[test]
    fn card_value_should_match_example_calculation() {
        let card = Card {
            id: 1,
            wins: [41, 48, 83, 86, 17].to_vec(),
            gots: [83, 86, 6, 31, 17, 9, 48, 53].to_vec(),
        };

        assert_eq!(card_value(&card), 8)
    }
}
