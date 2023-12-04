use std::{collections::HashMap, fs, isize};

fn main() {
    part2();
}

fn part1() {
    let input = fs::read_to_string("input.txt").expect("could not open input file");

    let schematic: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut part_set: Vec<u32> = Vec::new();

    let height = schematic.len();
    let width = schematic.first().unwrap().len();
    let dimensions = (width, height);

    for row in 0..height {
        let mut curr_num: Vec<char> = Vec::new();
        let mut touches_symbol = false;
        for col in 0..width {
            let curr = schematic[row][col];
            let is_digit = curr.is_digit(10);
            let irow = row as isize;
            let icol = col as isize;

            match (is_digit, touches_symbol, curr_num.is_empty()) {
                // Start of number, see if is valid, and add digit to curr_num
                (true, false, true) => {
                    curr_num.push(curr);
                    touches_symbol = vec![
                        (icol - 1, irow - 1),
                        (icol - 1, irow),
                        (icol - 1, irow + 1),
                        (icol, irow - 1),
                        (icol, irow + 1),
                    ]
                    .into_iter()
                    .any(|p| is_symbol(&schematic, dimensions, p));
                }
                // Middle of invalid number, see if is valid, and add digit to curr_num
                (true, false, false) => {
                    curr_num.push(curr);
                    touches_symbol = vec![(icol, irow - 1), (icol, irow + 1)]
                        .into_iter()
                        .any(|p| is_symbol(&schematic, dimensions, p));
                }
                // Middle of valid number, add digit to curr_num
                (true, true, false) => {
                    curr_num.push(curr);
                }
                // Reached end of number, see if is valid, if so add number to list, then reset
                (false, _, false) => {
                    touches_symbol = touches_symbol
                        || vec![(icol, irow - 1), (icol, irow), (icol, irow + 1)]
                            .into_iter()
                            .any(|p| is_symbol(&schematic, dimensions, p));

                    if touches_symbol {
                        let new_num_str: String = curr_num.into_iter().collect();
                        part_set.push(new_num_str.parse().unwrap());
                    }

                    curr_num = vec![];
                    touches_symbol = false;
                }
                _ => {}
            }
        }
    }

    println!("{}", part_set.iter().sum::<u32>());
}

fn part2() {
    let input = fs::read_to_string("input.txt").expect("could not open input file");

    let schematic: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut gears: HashMap<(usize, usize), [u32; 2]> = HashMap::new();

    let height = schematic.len();
    let width = schematic.first().unwrap().len();
    let dimensions = (width, height);

    for row in 0..height {
        let mut curr_num: Vec<char> = Vec::new();
        let mut curr_symbol: Option<(usize, usize)> = Option::None;
        for col in 0..width {
            let curr = schematic[row][col];
            let is_digit = curr.is_digit(10);
            let irow = row as isize;
            let icol = col as isize;

            match (is_digit, curr_symbol, curr_num.is_empty()) {
                // Start of number, see if is valid, and add digit to curr_num
                (true, None, _) => {
                    let symbol = if curr_num.is_empty() {
                        vec![
                            (icol - 1, irow - 1),
                            (icol - 1, irow),
                            (icol - 1, irow + 1),
                            (icol, irow - 1),
                            (icol, irow + 1),
                        ]
                    } else {
                        vec![(icol, irow - 1), (icol, irow + 1)]
                    }
                    .into_iter()
                    .find(|p| is_symbol(&schematic, dimensions, *p));

                    if let Some(s) = symbol {
                        curr_symbol = Some((s.0 as usize, s.1 as usize));
                    }

                    curr_num.push(curr);
                }
                // Middle of valid number, add digit to curr_num
                (true, Some(_), false) => {
                    curr_num.push(curr);
                }
                // Found a '.',
                // Potentially reached end of number, see if is valid,
                // if so add number to list, then reset
                (false, _, false) => {
                    let symbol = if curr_symbol.is_some() {
                        curr_symbol
                    } else {
                        vec![(icol, irow - 1), (icol, irow), (icol, irow + 1)]
                            .into_iter()
                            .find(|p| is_symbol(&schematic, dimensions, *p))
                            .and_then(|s| Some((s.0 as usize, s.1 as usize)))
                    };

                    if let Some(s) = symbol {
                        let new_num_str: String = curr_num.into_iter().collect();
                        let num = new_num_str.parse().unwrap();

                        if gears.contains_key(&s) {
                            gears.get_mut(&s).unwrap()[1] = num;
                        } else {
                            gears.insert(s, [num, 0]);
                        }
                    }

                    curr_num = vec![];
                    curr_symbol = None;
                }
                _ => {}
            }
        }
    }

    let sum: u32 = gears.into_iter().map(|(_, v)| v[0] * v[1]).sum();
    println!("{}", sum);
}

fn is_symbol(grid: &Vec<Vec<char>>, dimensions: (usize, usize), pos: (isize, isize)) -> bool {
    if pos.0 < 0 || pos.0 >= dimensions.0 as isize || pos.1 < 0 || pos.1 >= dimensions.1 as isize {
        return false;
    }

    let idx_pos = (pos.0 as usize, pos.1 as usize);

    let c = grid[idx_pos.1][idx_pos.0];
    !c.is_digit(10) && !c.eq(&'.')
}
