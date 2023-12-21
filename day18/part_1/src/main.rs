use std::{
    fmt::{self},
    fs,
};

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
struct Plan(Vec<Vec<u8>>);

impl fmt::Display for Plan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut plan = String::new();
        for row in &self.0 {
            let row_display: String = row.iter().map(|e| edge_char(e)).collect();
            plan += &row_display;
            plan += "\r\n";
        }
        write!(f, "{}", plan)
    }
}

fn edge_char(e: &u8) -> char {
    char::from_u32(match *e {
        NORTH_SOUTH => '│',
        NORTH_EAST => '└',
        NORTH_WEST => '┘',
        EAST_WEST => '─',
        SOUTH_EAST => '┌',
        SOUTH_WEST => '┐',
        _ => '.',
    } as u32)
    .unwrap()
}

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

        let width = min.0.unsigned_abs() + max.0 as usize;
        let height = min.1.unsigned_abs() + max.1 as usize;

        let origin = (min.0.unsigned_abs(), min.1.unsigned_abs());

        let mut plan: Vec<_> = vec![vec![0; width + 1]; height + 1];

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
                    plan[curr_edge.1 as usize][curr_edge.0 as usize] |= direction;
                    curr_edge = (curr_edge.0 + dx, curr_edge.1 + dy);
                    plan[curr_edge.1 as usize][curr_edge.0 as usize] |= opposite_direction;
                }

                new_curr
            });

        Plan(plan)
    }

    fn find_area(&self) -> usize {
        self.0.iter().map(|row| self.find_row_area(row)).sum()
    }

    fn find_row_area<'a>(&self, row: &Vec<u8>) -> usize {
        let mut waiting_on: Option<u8> = None;
        let mut inside = false;
        let mut area = 0;

        for edge in row {
            match *edge {
                NORTH_SOUTH => {
                    inside = !inside;
                    area += 1;
                }
                EAST_WEST => {
                    area += 1;
                }
                0 => {
                    if inside {
                        area += 1;
                    }
                    waiting_on = None;
                }
                _ => {
                    area += 1;
                    if let Some(edge_waited_on) = waiting_on {
                        if edge_waited_on == *edge {
                            inside = !inside;
                        } else {
                            waiting_on = None;
                        }
                    } else {
                        waiting_on = Some(edge ^ 0b1111);
                    }
                }
            }
        }

        area
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");

    let plan = Plan::from_str(&input, false);
    println!("part 1: {}", plan.find_area());
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

        let plan = Plan::from_str(input, false);
        //println!("{}", plan);
        let area = plan.find_area();

        assert_eq!(62, area);
    }

    #[test]
    fn test2() {
        let input = "R 2 X
D 4 X
R 2 X
U 4 X
R 2 X
D 6 X
L 2 X
D 2 X
L 2 X
U 2 X
L 2 X
U 6 X";

        let plan = Plan::from_str(input, false);

        println!("{}", plan);

        assert_eq!(51, plan.find_area());
    }
}
