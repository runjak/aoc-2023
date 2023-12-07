use std::{cmp::Ordering, collections::HashMap, error::Error, fs};

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

fn hand_value(hand: &Hand) -> u8 {
    let mut card_counts: HashMap<Card, u8> = HashMap::new();
    for card in hand.cards.iter() {
        card_counts.insert(*card, card_counts.get(card).unwrap_or(&0) + 1);
    }

    let first_group = card_counts.values().max().unwrap_or(&0);
    let second_group = card_counts
        .values()
        .filter(|x| x < &first_group)
        .max()
        .unwrap_or(&0);

    /*
      Five of a kind, where all five cards have the same label: AAAAA
      Four of a kind, where four cards have the same label and one card has a different label: AA8AA
      Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
      Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
      Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
      One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
      High card, where all cards' labels are distinct: 23456
    */
    match (first_group, second_group) {
        (5, _) => 7,
        (4, _) => 6,
        (3, 2) => 5,
        (3, _) => 4,
        (2, 2) => 3,
        (1, _) => 2,
        (_, _) => 1,
    }
}

fn cmp_hands_by_cards(hand1: &Hand, hand2: &Hand) -> Ordering {
    let mut card_order = hand1
        .cards
        .iter()
        .zip(hand2.cards.iter())
        .map(|(card1, card2)| card1.cmp(card2))
        .filter(|order| order != &Ordering::Equal);

    return card_order.next().unwrap_or(Ordering::Equal);
}

fn compare_hands(hand1: &Hand, hand2: &Hand) -> Ordering {
    let hand_order = hand_value(&hand1).cmp(&hand_value(&hand2));

    if hand_order != Ordering::Equal {
        return hand_order;
    }

    return cmp_hands_by_cards(hand1, hand2);
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/07/example-1.txt", "./inputs/07/input.txt"];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let mut hands = parse_hands(contents);

        hands.sort_by(compare_hands);

        let sum = hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| {
                let rank = rank + 1;
                hand.bet * i32::try_from(rank).unwrap()
            })
            .sum::<i32>();

        println!("{}", sum);

        // break;
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
