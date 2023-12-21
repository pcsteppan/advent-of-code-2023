use rayon::prelude::*;
use std::{collections::HashMap, fs};

use crate::edge::*;

pub mod edge {
    pub const NORTH: u8 = 0b0001;
    pub const EAST: u8 = 0b0010;
    pub const SOUTH: u8 = 0b0100;
    pub const WEST: u8 = 0b1000;
    pub const NORTH_SOUTH: u8 = NORTH | SOUTH;
    pub const EAST_WEST: u8 = EAST | WEST;
    pub const NORTH_EAST: u8 = NORTH | EAST;
    pub const NORTH_WEST: u8 = NORTH | WEST;
    pub const SOUTH_EAST: u8 = SOUTH | EAST;
    pub const SOUTH_WEST: u8 = SOUTH | WEST;
}

#[derive(Debug)]
struct Plan(Vec<HashMap<usize, u8>>);

impl Plan {
    fn from_str(str: &str, use_hex: bool) -> Self {
        let instructions: Vec<_> = str
            .lines()
            .map(|l| {
                let mut words = l.split_whitespace();
                let dir = words.next().unwrap();
                let edge_dir = match dir.chars().next().unwrap() {
                    'U' => NORTH,
                    'R' => EAST,
                    'D' => SOUTH,
                    'L' => WEST,
                    _ => panic!("unexpected direction while parsing instructions"),
                };
                let p1_length: usize = words.next().unwrap().parse().unwrap();
                let color = words.next().unwrap().replace(['(', '#', ')'], "");

                let length = if use_hex {
                    usize::from_str_radix(&color[0..5], 16).unwrap()
                } else {
                    p1_length
                };

                let dir = if use_hex {
                    match usize::from_str_radix(&color.chars().nth(5).unwrap().to_string(), 16)
                        .unwrap()
                    {
                        0 => EAST,
                        1 => SOUTH,
                        2 => WEST,
                        3 => NORTH,
                        _ => panic!("parsing unknown direction from hex string"),
                    }
                } else {
                    edge_dir
                };

                (dir, length)
            })
            .collect();

        let (max, min, _): ((isize, isize), (isize, isize), _) = instructions.iter().fold(
            ((0, 0), (0, 0), (0, 0)),
            |(max, min, curr), (direction, length)| {
                let new_curr: (isize, isize) = match *direction {
                    NORTH => (curr.0, curr.1 - *length as isize),
                    EAST => (curr.0 + *length as isize, curr.1),
                    SOUTH => (curr.0, curr.1 + *length as isize),
                    WEST => (curr.0 - *length as isize, curr.1),
                    _ => panic!("unexpected direction found while finding dimensions of plan"),
                };

                (
                    (max.0.max(new_curr.0), max.1.max(new_curr.1)),
                    (min.0.min(new_curr.0), min.1.min(new_curr.1)),
                    new_curr,
                )
            },
        );

        let height = min.1.unsigned_abs() + max.1 as usize;

        let origin = (min.0.unsigned_abs(), min.1.unsigned_abs());

        let mut plan: Vec<_> = vec![HashMap::new(); height + 1];

        instructions
            .iter()
            .fold(origin, |curr, (direction, length)| {
                let new_curr = match *direction {
                    NORTH => (curr.0, curr.1 - length),
                    EAST => (curr.0 + length, curr.1),
                    SOUTH => (curr.0, curr.1 + length),
                    WEST => (curr.0 - length, curr.1),
                    _ => panic!("unexpected direction found while finding dimensions of plan"),
                };

                let (dx, dy): (isize, isize) = match *direction {
                    NORTH => (0, -1),
                    EAST => (1, 0),
                    SOUTH => (0, 1),
                    WEST => (-1, 0),
                    _ => panic!("unexpected direction found while finding dimensions of plan"),
                };

                let mut curr_edge = (curr.0 as isize, curr.1 as isize); //.clone() as (isize, isize);
                let opposite_direction = match *direction {
                    SOUTH => NORTH,
                    NORTH => SOUTH,
                    EAST => WEST,
                    WEST => EAST,
                    _ => panic!(),
                };

                while curr_edge.0 != new_curr.0 as isize || curr_edge.1 != new_curr.1 as isize {
                    plan[curr_edge.1 as usize]
                        .entry(curr_edge.0 as usize)
                        .and_modify(|e| *e = &*e | *direction)
                        .or_insert(*direction);
                    curr_edge = (curr_edge.0 + dx, curr_edge.1 + dy);
                    plan[curr_edge.1 as usize]
                        .entry(curr_edge.0 as usize)
                        .and_modify(|e| *e = &*e | opposite_direction)
                        .or_insert(opposite_direction);
                }

                new_curr
            });

        Plan(plan)
    }

    fn find_area(&self) -> usize {
        self.0.par_iter().map(|row| self.find_row_area(row)).sum()
    }

    fn find_row_area<'a>(&self, row: &HashMap<usize, u8>) -> usize {
        let mut waiting_on: Option<u8> = None;
        let mut inside = false;
        let mut area = 0;
        let mut prev_col = 0;

        let mut edge_indexes: Vec<&usize> = row.keys().collect();
        edge_indexes.sort();

        for edge_index in edge_indexes {
            let edge = row.get(edge_index).unwrap();

            match *edge {
                NORTH_SOUTH => {
                    area += 1;
                    if inside && *edge_index != 0 {
                        area += edge_index - prev_col - 1;
                    }
                    inside = !inside;
                }
                EAST_WEST => {
                    area += 1;
                }
                NORTH_WEST | NORTH_EAST | SOUTH_EAST | SOUTH_WEST => {
                    area += 1;
                    if inside {
                        area += edge_index - prev_col - 1;
                    }
                    if let Some(edge_waited_on) = waiting_on {
                        if edge_waited_on == *edge {
                            inside = !inside;
                        }
                        waiting_on = None;
                    } else {
                        waiting_on = Some(edge ^ 0b1111);
                    }
                }
                _ => {
                    panic!("parsing unexpected u8 while finding area of row: {}", edge);
                }
            }

            prev_col = *edge_index;
        }
        area
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");

    let plan = Plan::from_str(&input, true);
    println!("part 2: {}", plan.find_area());
}

#[cfg(test)]
mod test {
    use crate::Plan;

    #[test]
    fn test1() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let plan = Plan::from_str(input, true);
        let area = plan.find_area();

        println!("difference: {}", area - 952408144115);
        assert_eq!(952408144115, area);
    }

    #[test]
    fn test2() {
        let input = "R 6 (#000020)
D 5 (#000021)
L 2 (#000022)
D 2 (#000023)";

        let plan = Plan::from_str(input, true);
        let area = plan.find_area();

        assert_eq!(9, area);
    }
}
