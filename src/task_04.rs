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

// Count cards and instances as per the second puzzle of the task
fn count_cards(cards: Vec<Card>) -> u32 {
    let mut total_cards: u32 = 0;

    let mut cards_and_counts = cards.iter().map(|card| (card, 1_u32)).collect::<Vec<_>>();
    let mut cards_and_counts: &mut [(&Card, u32)] = &mut cards_and_counts[..];

    while cards_and_counts.len() > 0 {
        let (first_card, first_card_count) = cards_and_counts[0];
        cards_and_counts = &mut cards_and_counts[1..];

        total_cards += first_card_count;

        let matching_values: usize = count_matching_values(first_card)
            .try_into()
            .unwrap_or_default();
        for card_index in 0..matching_values {
            let (card, card_count) = cards_and_counts[card_index];

            cards_and_counts[card_index] = (card, card_count);
        }
    }

    return total_cards;
}

pub fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/04/example-1.txt", "./inputs/04/input.txt"];

    for path in paths {
        println!("Reading file {}.", path);
        let contents = fs::read_to_string(path)?;

        let cards = contents.lines().flat_map(parse_card).collect::<Vec<_>>();

        println!("Number of cards won: {}", count_cards(cards));
        break;
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("04-1:");
    first()?;
    println!("04-2:");
    second()?;

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
