use std::{fs, collections::HashSet};

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    count: u32,
    winning_numbers: Vec<u32>,
    game_numbers: Vec<u32>,
}

impl Card {
    fn get_matches(&self) -> u32 {
        let winning_numbers_set: HashSet<_> = self.winning_numbers.iter().collect();
        let game_numbers_set: HashSet<_> = self.game_numbers.iter().collect();

        let matches = winning_numbers_set.intersection(&game_numbers_set).cloned().collect::<Vec<_>>().len();
        matches as u32 
    }

    fn get_wins(&self) -> u32 {
        let matches = self.get_matches();
        if matches > 0 { (1 << matches - 1) as u32 } else { 0 }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input file");

    let mut cards: Vec<Card> = input.lines().map(|l| {
        let (card_part, number_part) = l.split_once(":").unwrap();
        let card_id = card_part.split_once(" ").unwrap().1.trim().parse::<u32>().unwrap();
        let (winning_numbers, game_numbers) = number_part.split_once("|").unwrap();
        
        Card {
            id: card_id,
            count: 1, 
            winning_numbers: winning_numbers.trim().split(" ").map(|n| n.parse::<u32>().unwrap_or_default()).filter(|n| n > &0).collect(),
            game_numbers: game_numbers.trim().split(" ").map(|n| n.parse::<u32>().unwrap_or_default()).filter(|n| n > &0).collect(),
        }
    }).collect();

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for i in 0..cards.len() {
        let curr = &cards[i].clone();
        let matches = curr.get_matches();

        for j in 0..matches {
            cards[1+i+j as usize].count += curr.count;
        }

        part1_sum += curr.get_wins();
        part2_sum += curr.count;
    } 
   
    println!("point total: {}", part1_sum); // 23235
    println!("card count total: {}", part2_sum);
}
