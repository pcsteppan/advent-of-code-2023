use std::{collections::HashMap, str::CharIndices};

fn main() {
    part2();
}

fn part1() {
    let content = 
        std::fs::read_to_string("./src/input.txt")
        .expect("could not read file");
    
    let sum: u32 = 
        content.lines()
            .map(|l| {
                let mut num_chars = l.chars().filter(|c| c.is_numeric());
                let head = num_chars.next().unwrap();
                let tail = num_chars.last().unwrap_or(head);

                head.to_digit(10).unwrap() * 10 
                    + tail.to_digit(10).unwrap()
            })
            .sum();
    
    println!("{}", sum);
}

fn part2() {
    let content = 
        std::fs::read_to_string("./src/test.txt")
        .expect("could not read file");

    let map = HashMap::from([
        ("1", 1), ("one", 1),
        ("2", 2), ("two", 2),
        ("3", 3), ("three", 3),
        ("4", 4), ("four", 4),
        ("5", 5), ("five", 5),
        ("6", 6), ("six", 6),
        ("7", 7), ("seven", 7),
        ("8", 8), ("eight", 8),
        ("9", 9), ("nine", 9),
    ]);
    
    let sum: i32 = 
        content.lines()
            .map(|l| {
                let indices = find_indices(l, &map);
                let head = map.get(&(indices.first().unwrap().1.as_str())).unwrap();
                let tail = map.get(&(indices.last().unwrap().1.as_str())).unwrap();
                println!("{} - {:?} - {}", head, indices, tail);
                head * 10 + tail
            })
            .sum();
    
    println!("{}", sum);
}

fn find_indices(line: &str, map: &HashMap::<&str, i32>) -> Vec<(usize, String)> {
    let match_indices: Vec<_> = map.keys().map(|key| {
        line.match_indices(key).map(|r| (r.0, r.1.to_string())).collect::<Vec<_>>()
    }).collect();
    
    let mut results: Vec<_> = match_indices.into_iter().flatten().collect();
    results.sort_by(|a, b| a.0.cmp(&b.0));
    results
}