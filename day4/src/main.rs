use std::{fs, collections::HashSet};

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    game_numbers: Vec<u32>,
}

impl Card {
    fn get_wins(&self) -> u32 {
        let winning_numbers_set: HashSet<_> = self.winning_numbers.iter().collect();
        let game_numbers_set: HashSet<_> = self.game_numbers.iter().collect();

        let wins = winning_numbers_set.intersection(&game_numbers_set).cloned().collect::<Vec<_>>().len();
        if wins > 0 { (1 << wins - 1) as u32 } else { 0 }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input file");

    let cards: Vec<Card> = input.lines().map(|l| {
        let (card_part, number_part) = l.split_once(":").unwrap();
        let card_id = card_part.split_once(" ").unwrap().1.trim().parse::<u32>().unwrap();
        let (winning_numbers, game_numbers) = number_part.split_once("|").unwrap();
        
        Card {
            id: card_id,
            winning_numbers: winning_numbers.trim().split(" ").map(|n| n.parse::<u32>().unwrap_or_default()).filter(|n| n > &0).collect(),
            game_numbers: game_numbers.trim().split(" ").map(|n| n.parse::<u32>().unwrap_or_default()).filter(|n| n > &0).collect(),
        }
    }).collect();

    let sum : u32 = cards.iter().map(|c| c.get_wins()).sum();
    println!("{}", sum); // 23235
}
