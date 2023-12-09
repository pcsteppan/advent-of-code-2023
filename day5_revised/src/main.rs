use std::collections::HashMap;
use std::fs;
use std::ops::Range;

#[derive(Debug, Clone)]
struct Layer {
    range_maps: Vec<RangeMap>,
    source: String,
    destination: String,
}

impl Layer {
    fn compose(self, other: &Layer, allow_fallthrough: bool) -> Layer {
        Layer {
            source: self.source,
            destination: other.destination.clone(),
            range_maps: self.range_maps.into_iter().flat_map(|range_map| range_map.transform(other, allow_fallthrough)).collect(), 
        }
    }
}

#[derive(Debug, Clone)]
struct RangeMap {
    range: Range<i64>,
    offset: i64
}

impl RangeMap {
    fn transform(self, layer: &Layer, allow_fallthrough: bool) -> Vec<RangeMap> {
        let mut stack = vec![self];
        let mut results: Vec<RangeMap> = vec![];

        while !stack.is_empty() {
            let curr = stack.pop().unwrap();
            println!("stack: {:?}\r\ncurr: {:?}", stack, curr);

            for curr_map in layer.range_maps.as_slice() {
                println!("comparing curr {:?}: to curr_map: {:?}", curr, curr_map);
                let combined_offset = curr_map.offset + curr.offset;

                let left_inside = curr_map.range.contains(&curr.range.start);
                let right_inside = curr_map.range.contains(&curr.range.end);
                let has_no_overlap = curr.range.end < curr_map.range.start || curr.range.start > curr_map.range.end;

                if has_no_overlap {
                    continue;        
                }

                match (left_inside, right_inside) {
                    (false, false) => {
                        // split into three ranges
                        stack.push(RangeMap { range: curr.range.start..curr_map.range.start-1, offset: curr.offset });
                        results.push(RangeMap { range: curr_map.range.clone(), offset: combined_offset });
                        stack.push(RangeMap { range: curr_map.range.end+1..curr.range.end, offset: curr.offset });
                        break;
                    },
                    (false, true) => {
                        // split into two ranges
                        stack.push(RangeMap { range: curr.range.start..curr_map.range.start-1, offset: curr.offset });
                        results.push(RangeMap { range: curr_map.range.start..curr.range.end, offset: combined_offset });
                        break;
                    },
                    (true, false) => {
                        // split into two ranges
                        results.push(RangeMap { range: curr.range.start..curr_map.range.end, offset: combined_offset });
                        stack.push(RangeMap { range: curr_map.range.end+1..curr.range.end, offset: curr.offset });
                        break;
                    },
                    (true, true) => {
                        // range stays the same, just update the offset
                        results.push(RangeMap { range: curr.range.clone(), offset: combined_offset});
                        break;
                    }
                }
            }

            if allow_fallthrough {
                results.push(curr);
            }
        }

        results
    }
}

impl System {
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSystemError;

#[derive(Debug)]
struct System {
    layers: HashMap<String, Layer>,
}

impl System {
    fn compose_all_layers(&self) -> Layer {
        // collapse all layers
        let mut composition = self.layers.get("source").unwrap().clone();

        while self.layers.contains_key(&composition.destination) {
            let next_layer = self.layers.get(&composition.destination).unwrap();
            println!("composing layer {} with layer {}", composition.source, composition.destination);

            composition = composition.clone().compose(next_layer, composition.destination.eq("seed"));
            composition.range_maps = composition.range_maps.into_iter().map(|rm| RangeMap {
                range: rm.range.start+rm.offset..rm.range.end+rm.offset,
                offset: 0
            }).collect();
            dbg!(composition.clone());
        }

        composition
    }

    fn from_str(s: &str) -> Result<System, ParseSystemError> {
        let mut alpha_layer = Layer {
            source: "source".to_string(),
            destination: "seed".to_string(),
            range_maps: vec![],
        };
        let mut maps = HashMap::new();

        let mut curr_src = "";
        let mut curr_dest = "";

        for line in s.lines() {
            if line.contains("seeds") {
                // e.g., 'seeds: 12 13 14'
                
                let mut seed_input: Vec<i64> = line
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .into_iter()
                    .collect();

                for chunk in seed_input.chunks_mut(2) {
                    alpha_layer.range_maps.push(RangeMap {
                        range: chunk[0]..chunk[0]+chunk[1], 
                        offset: 0
                    })
                }
                
            } else if line.contains("map") {
                // e.g., 'soil-to-fertilizer map:'
                (curr_src, curr_dest) = line.split_once(" ").unwrap().0.split_once("-to-").unwrap();
                maps.insert(
                    curr_src.to_string(),
                    Layer {
                        source: curr_src.to_string(),
                        destination: curr_dest.to_string(),
                        range_maps: vec![],
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
                maps.get_mut(curr_src).unwrap().range_maps.push(new_map);
            }
        }

        maps.insert("source".to_string(), alpha_layer);

        Ok(System { layers: maps })
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not open input");

    let system = System::from_str(&input).unwrap();

//    dbg!(system);

    let composite_system = system.compose_all_layers();
    dbg!(composite_system);
}

#[cfg(test)]
mod test {
    use crate::{RangeMap, Layer, System};

    #[test]
    fn test_transform() {
        let test_input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let system = System::from_str(test_input).unwrap();

        let composition = system.compose_all_layers();

        let min = composition.range_maps.iter().map(|i| i.range.start + i.offset).min().unwrap();
        println!("min {}", min);
    }
}

