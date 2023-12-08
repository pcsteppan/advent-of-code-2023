use rayon::prelude::*;
use std::{collections::HashMap, fs, ops::Range, str::FromStr};

#[derive(Debug, Clone)]
struct RangeMap {
    range: Range<i64>,
    offset: i64,
}

#[derive(Debug, Clone)]
struct Map {
    source: String,
    destination: String,
    functions: Vec<RangeMap>,
}

impl Map {
    fn get(self: &Self, n: &i64) -> i64 {
        let containing_map = self.functions.iter().find(|f| f.range.contains(&n));

        if let Some(map) = containing_map {
            return n + map.offset;
        }

        *n
    }
}

#[derive(Debug)]
struct System {
    seeds: Vec<i64>,
    maps: HashMap<String, Map>,
}

impl System {
    fn get_seed_locations(self: &Self) -> Vec<i64> {
        // start by finding seed map
        // follow through until we map to location
        //

        let mut curr_src = "seed";
        let mut seed_transforms = self.seeds.clone();
        while self.maps.get(curr_src).is_some() {
            println!("working on: {}", curr_src);
            let curr_map = self.maps.get(curr_src).unwrap();
            seed_transforms = seed_transforms
                .into_par_iter()
                .map(|seed| curr_map.get(&seed))
                .collect();

            curr_src = &curr_map.destination;
        }

        seed_transforms
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSystemError;

impl System {
    fn from_str(s: &str, is_part_2: bool) -> Result<System, ParseSystemError> {
        let mut maps = HashMap::new();
        let mut seeds = vec![];

        let mut curr_src = "";
        let mut curr_dest = "";

        for line in s.lines() {
            if line.contains("seeds") {
                // e.g., 'seeds: 12 13 14'
                if is_part_2 {
                    let mut seed_input: Vec<i64> = line
                        .split_once(": ")
                        .unwrap()
                        .1
                        .split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .into_iter()
                        .collect();

                    for chunk in seed_input.chunks_mut(2) {
                        let mut chunk_seeds: Vec<i64> = (chunk[0]..chunk[0] + chunk[1]).collect();
                        seeds.append(&mut chunk_seeds);
                    }
                } else {
                    seeds = line
                        .split_once(": ")
                        .unwrap()
                        .1
                        .split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect();
                }
            } else if line.contains("map") {
                // e.g., 'soil-to-fertilizer map:'
                (curr_src, curr_dest) = line.split_once(" ").unwrap().0.split_once("-to-").unwrap();
                maps.insert(
                    curr_src.to_string(),
                    Map {
                        source: curr_src.to_string(),
                        destination: curr_dest.to_string(),
                        functions: vec![],
                    },
                );
            } else if !line.is_empty() {
                // e.g., '0 10 12'
                let mut line_data = line.split_whitespace().map(|n| n.parse::<i64>().unwrap());
                let dest_start = line_data.next().unwrap();
                let src_start = line_data.next().unwrap();
                let range = line_data.next().unwrap();
                let new_map = RangeMap {
                    range: src_start..src_start + range,
                    offset: dest_start - src_start,
                };
                maps.get_mut(curr_src).unwrap().functions.push(new_map);
            }
        }

        Ok(System { maps, seeds })
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not open input");

    // part 1
    let system = System::from_str(&input, false).unwrap();

    let locations = system.get_seed_locations();
    println!("part 1: {}", locations.iter().min().unwrap());

    // part 2
    let system2 = System::from_str(&input, true).unwrap();

    let locations2 = system2.get_seed_locations();
    println!("part2: {}", locations2.par_iter().min().unwrap());
}
