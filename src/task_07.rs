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

#[derive(Debug, PartialEq)]
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

fn count_cards(hand: &Hand) -> HashMap<Card, u8> {
    let mut card_counts: HashMap<Card, u8> = HashMap::new();
    for card in hand.cards.iter() {
        card_counts.insert(*card, card_counts.get(card).unwrap_or(&0) + 1);
    }

    return card_counts;
}

fn hand_value(hand: &Hand) -> u8 {
    let card_counts = count_cards(hand);

    let (first_key, first_group) = card_counts
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap_or((&0, &0));

    let second_group = card_counts
        .iter()
        .filter(|(key, _value)| *key != first_key)
        .map(|a| a.1)
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
        (2, _) => 2,
        (_, _) => 1,
    }
}

fn cmp_cards(cards1: &Vec<Card>, cards2: &Vec<Card>) -> Ordering {
    cards1
        .iter()
        .zip(cards2.iter())
        .map(|(card1, card2)| card1.cmp(card2))
        .filter(|order| order != &Ordering::Equal)
        .next()
        .unwrap_or(Ordering::Equal)
}

fn cmp_hands_by_cards(hand1: &Hand, hand2: &Hand) -> Ordering {
    cmp_cards(&hand1.cards, &hand2.cards)
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

        let bets = hands.iter().map(|hand| hand.bet);
        let total_winnings = bets
            .enumerate()
            .map(|(rank, bet)| {
                let rank = rank + 1;
                bet * i32::try_from(rank).unwrap()
            })
            .sum::<i32>();

        println!("{}", total_winnings);
    }

    Ok(())
}

fn fill_jokers(hand: &Hand) -> Hand {
    let joker = 11;
    let ace = 14;

    let card_counts = count_cards(hand);

    let (max_card, max_card_count) = card_counts
        .iter()
        .filter(|(card, _)| **card != joker)
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap_or((&0, &0));

    let chosen_card = if max_card_count > &1 { max_card } else { &ace };

    let cards = hand
        .cards
        .iter()
        .map(|current| {
            if *current == joker {
                *chosen_card
            } else {
                *current
            }
        })
        .collect::<Vec<_>>();

    Hand {
        cards,
        bet: hand.bet,
    }
}

fn compare_hands_with_jokers(hand1: &Hand, hand2: &Hand) -> Ordering {
    let hand_order = hand_value(&fill_jokers(hand1)).cmp(&hand_value(&fill_jokers(hand2)));

    if hand_order != Ordering::Equal {
        return hand_order;
    }

    let joker = &11;
    let weakest = 1;

    let joker_cards_1 = hand1
        .cards
        .iter()
        .map(|card| if card == joker { weakest } else { *card })
        .collect::<Vec<_>>();
    let joker_cards_2 = hand2
        .cards
        .iter()
        .map(|card| if card == joker { weakest } else { *card })
        .collect::<Vec<_>>();

    cmp_cards(&joker_cards_1, &joker_cards_2)
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/07/example-1.txt", "./inputs/07/input.txt"];

    for path in paths {
        let contents = fs::read_to_string(path)?;
        let mut hands = parse_hands(contents);

        hands.sort_by(compare_hands_with_jokers);

        let bets = hands.iter().map(|hand| hand.bet);
        let total_winnings = bets
            .enumerate()
            .map(|(rank, bet)| {
                let rank = rank + 1;
                bet * i32::try_from(rank).unwrap()
            })
            .sum::<i32>();

        println!("{}", total_winnings);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("07-1:");
    first()?;
    println!("07-2:");
    second()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::task_07::{fill_jokers, hand_value, parse_hands, Hand};

    #[test]
    fn hand_value_should_behave() {
        // Five of a kind
        assert_eq!(
            hand_value(&Hand {
                bet: 0,
                cards: [1, 1, 1, 1, 1].to_vec()
            }),
            7
        );
        // Four of a kind
        assert_eq!(
            hand_value(&Hand {
                bet: 0,
                cards: [1, 1, 1, 1, 2].to_vec()
            }),
            6
        );
        // Full house
        assert_eq!(
            hand_value(&Hand {
                bet: 0,
                cards: [1, 1, 1, 2, 2].to_vec()
            }),
            5
        );
        // 3 of a kind
        assert_eq!(
            hand_value(&Hand {
                bet: 0,
                cards: [1, 1, 1, 2, 3].to_vec()
            }),
            4
        );
        // Two pairs
        assert_eq!(
            hand_value(&Hand {
                bet: 0,
                cards: [1, 1, 2, 2, 3].to_vec()
            }),
            3
        );
        // One pair
        assert_eq!(
            hand_value(&Hand {
                bet: 0,
                cards: [1, 1, 2, 3, 4].to_vec()
            }),
            2
        );
        // High card
        assert_eq!(
            hand_value(&Hand {
                bet: 0,
                cards: [5, 4, 3, 2, 1].to_vec()
            }),
            1
        );
    }

    #[test]
    fn fill_jokers_should_behave() {
        let contents = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .join("\n");

        let hands = parse_hands(contents);
        let filled_hands = hands
            .iter()
            .map(|hand| fill_jokers(hand))
            .collect::<Vec<_>>();

        let contents = [
            "32T3K 765",
            "T5555 684",
            "KK677 28",
            "KTTTT 220",
            "QQQQA 483",
        ]
        .join("\n");

        let expected_hands = parse_hands(contents);

        assert_eq!(filled_hands, expected_hands);
    }

    #[test]
    fn fill_jokers_should_fill_jokers_in_place() {
        let input = Hand {
            cards: [2, 14, 11, 2, 14].to_vec(),
            bet: 0,
        };

        let expected = [2, 14, 14, 2, 14].to_vec();
        let actual = fill_jokers(&input).cards;

        assert_eq!(actual, expected);
    }
}
