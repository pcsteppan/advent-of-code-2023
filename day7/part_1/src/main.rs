use itertools::Itertools;
use std::{collections::HashSet, fs};

#[derive(Debug)]
enum CardType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Card {
    card_type: CardType,
    card_value: usize,
    score_value: usize,
}

impl Card {
    fn from_str(str: &str, score_value: usize) -> Card {
        let card_chars = str.chars();
        let distinct_card_set: HashSet<char> = HashSet::from_iter(card_chars.clone());
        let counts = card_chars.into_iter().counts();
        let distinct_count = distinct_card_set.len();
        let most_frequent_count = counts.values().max().unwrap();

        println!("{} {}", distinct_count, most_frequent_count);

        let card_type = match (distinct_count, most_frequent_count) {
            (1, _) => CardType::FiveOfAKind,
            (2, 4) => CardType::FourOfAKind,
            (2, 3) => CardType::FullHouse,
            (3, 3) => CardType::ThreeOfAKind,
            (3, 2) => CardType::TwoPair,
            (4, 2) => CardType::OnePair,
            (5, 1) => CardType::HighCard,
            _ => {
                panic!("unexpected card type: {}", str);
            }
        };

        let card_type_prefix = match card_type {
            CardType::FiveOfAKind => 'f',
            CardType::FourOfAKind => 'e',
            CardType::FullHouse => 'd',
            CardType::ThreeOfAKind => 'c',
            CardType::TwoPair => 'b',
            CardType::OnePair => 'a',
            CardType::HighCard => '9',
        };

        let hex_card = card_type_prefix.to_string()
            + &str
                .replace('T', "a")
                .replace('J', "b")
                .replace('Q', "c")
                .replace('K', "d")
                .replace('A', "e");

        let hex_card_value = usize::from_str_radix(&hex_card, 16).unwrap();

        Card {
            card_type,
            card_value: hex_card_value,
            score_value,
        }
    }
}

fn solve(input: &str) -> usize {
    let mut cards: Vec<Card> = input
        .lines()
        .map(|l| {
            let (hand, val) = l.split_once(" ").unwrap();

            Card::from_str(hand, val.parse().unwrap())
        })
        .collect();

    cards.sort_by(|a, b| a.card_value.cmp(&b.card_value));

    let result: usize = cards
        .iter()
        .enumerate()
        .map(|(i, c)| (i + 1) * c.score_value)
        .sum();

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input.txt");
    let result = solve(&input);
    println!("part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = solve(input);

        assert_eq!(result, 6440);
    }
}
