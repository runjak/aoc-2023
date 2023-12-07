use std::{cmp::Ordering, error::Error, fs};

type Card = i32;

fn parse_cards(cards: &str) -> Vec<Card> {
    cards
        .chars()
        .filter_map(|char| -> Option<Card> {
            match char {
                'A' => Some(14),
                'K' => Some(13),
                'Q' => Some(12),
                'J' => Some(11),
                'T' => Some(10),
                '9' => Some(9),
                '8' => Some(8),
                '7' => Some(7),
                '6' => Some(6),
                '5' => Some(5),
                '4' => Some(4),
                '3' => Some(3),
                '2' => Some(2),
                _ => None,
            }
        })
        .collect()
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bet: i32,
}

fn parse_hands(contents: String) -> Vec<Hand> {
    contents
        .lines()
        .filter_map(|line| -> Option<Hand> {
            let parts = line.split(" ").collect::<Vec<_>>();
            let [cards, bet] = parts.as_slice() else {
                return None;
            };

            let cards = parse_cards(cards);
            let bet = bet.parse::<i32>().ok()?;

            Some(Hand { cards, bet })
        })
        .collect()
}

fn hand_value(hand: Hand) -> u8 {
    /*
      Five of a kind, where all five cards have the same label: AAAAA
      Four of a kind, where four cards have the same label and one card has a different label: AA8AA
      Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
      Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
      Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
      One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
      High card, where all cards' labels are distinct: 23456
    */

    return 0;
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/07/example-1.txt", "./inputs/07/input.txt"];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let mut hands = parse_hands(contents);

        println!("Got hands: {:?}", hands);

        hands.sort_by(|hand1, hand2| -> Ordering { hand1.bet.cmp(&hand2.bet) });

        break;
    }

    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    println!("To be implemented.");

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("07-1:");
    first()?;
    println!("07-2:");
    second()?;

    Ok(())
}
