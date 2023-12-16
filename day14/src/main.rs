use std::{collections::HashMap, fs, time::Instant, usize::MAX};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Thing {
    Rock,
    Pillar,
    Empty,
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Thing {
    fn from_char(c: char) -> Thing {
        match c {
            '.' => Thing::Empty,
            'O' => Thing::Rock,
            '#' => Thing::Pillar,
            _ => panic!("unexpected char: {}", c),
        }
    }
}

#[derive(Debug, Clone)]
struct Data {
    grid: Vec<Vec<Thing>>,
    width: usize,
    height: usize,
}

impl Data {
    fn from_str(str: &str) -> Data {
        let grid: Vec<Vec<Thing>> = str
            .lines()
            .map(|l| l.chars().map(Thing::from_char).collect())
            .collect();

        let height = grid.len();
        let width = grid[0].len();

        Data {
            grid,
            width,
            height,
        }
    }

    fn get_load(&self) -> usize {
        let transpose: Vec<Vec<Thing>> = (0..self.grid[0].len())
            .map(|col| {
                (0..self.grid.len())
                    .map(|row| self.grid[row][col])
                    .collect()
            })
            .collect();

        transpose.iter().map(get_col_load).sum()
    }

    fn cycle(&mut self) {
        vec![
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .into_iter()
        .for_each(|direction| self.tilt(direction));
    }

    fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::North => (0..self.width).for_each(|col| self.sort_col(col, 0, self.height)),
            Direction::East => {
                range(self.height, 0).for_each(|row| self.sort_row(row, self.width, 0))
            }
            Direction::South => {
                range(self.width, 0).for_each(|col| self.sort_col(col, self.height, 0))
            }
            Direction::West => (0..self.height).for_each(|row| self.sort_row(row, 0, self.width)),
        };
    }

    fn sort_row(&mut self, row: usize, start: usize, end: usize) {
        let mut end_of_segment_found = false;
        let mut prev_rock_idx_in_segment_found = MAX;

        for i in range(start, end) {
            if end_of_segment_found || self.grid[row][i] != Thing::Empty {
                if self.grid[row][i] == Thing::Pillar {
                    end_of_segment_found = false;
                    prev_rock_idx_in_segment_found = MAX;
                }
                continue;
            }

            let sub_loop_start = if prev_rock_idx_in_segment_found != MAX {
                prev_rock_idx_in_segment_found
            } else {
                i
            };

            for j in range(sub_loop_start, end) {
                if self.grid[row][j] == Thing::Rock {
                    prev_rock_idx_in_segment_found = j;
                    self.grid[row].swap(i, j);
                    break;
                } else if self.grid[row][j] == Thing::Pillar {
                    end_of_segment_found = true;
                    break;
                }
            }
        }
    }

    fn sort_col(&mut self, col: usize, start: usize, end: usize) {
        let mut end_of_segment_found = false;
        let mut prev_rock_idx_in_segment_found = MAX;
        for i in range(start, end) {
            if end_of_segment_found || self.grid[i][col] != Thing::Empty {
                if self.grid[i][col] == Thing::Pillar {
                    end_of_segment_found = false;
                    prev_rock_idx_in_segment_found = MAX;
                }
                continue;
            }

            let sub_loop_start = if prev_rock_idx_in_segment_found != MAX {
                prev_rock_idx_in_segment_found
            } else {
                i
            };

            for j in range(sub_loop_start, end) {
                if self.grid[j][col] == Thing::Rock {
                    prev_rock_idx_in_segment_found = j;
                    let temp = self.grid[i][col];
                    self.grid[i][col] = self.grid[j][col];
                    self.grid[j][col] = temp;
                    break;
                } else if self.grid[j][col] == Thing::Pillar {
                    end_of_segment_found = true;
                    break;
                }
            }
        }
    }
}

fn range(start: usize, end: usize) -> Box<dyn Iterator<Item = usize>> {
    if start < end {
        Box::new(start..end)
    } else {
        Box::new((end..start).rev())
    }
}

fn get_col_load(col: &Vec<Thing>) -> usize {
    col.iter().enumerate().fold(0, |acc, (i, curr)| {
        if *curr == Thing::Rock {
            acc + (col.len() - i)
        } else {
            acc
        }
    })
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input.txt");

    let mut data = Data::from_str(&input);
    data.tilt(Direction::North);

    println!("part 1: {}", data.get_load());

    let start = Instant::now();
    let mut difference_hashmap: HashMap<Vec<[usize; 2]>, usize> = HashMap::new();
    let mut last_data = data.clone();
    let mut cycle_len: Option<usize> = None;
    for i in 0..1_000_000_000 {
        if let Some(cl) = cycle_len {
            if i % cl == 1_000_000_000 % cl {
                break;
            }
        }

        data.cycle();

        let diff: Vec<[usize; 2]> = last_data
            .grid
            .iter()
            .enumerate()
            .zip(&data.grid)
            .flat_map(|((row_i, row1), row2)| {
                row1.iter()
                    .enumerate()
                    .zip(row2)
                    .filter(|((_, t1), t2)| *t1 != *t2)
                    .map(|((j, _), _)| [row_i, j])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<[usize; 2]>>();

        if let Some(prev_index_of_identical_diff) = difference_hashmap.get(&diff) {
            cycle_len = Some(i - prev_index_of_identical_diff);
        } else {
            difference_hashmap.insert(diff, i);
        }

        last_data = data.clone();
    }

    println!(
        "part 2: {}, completed in: {:?}",
        data.get_load(),
        start.elapsed()
    );
}

#[cfg(test)]
mod test {
    use crate::{range, Data, Direction};

    #[test]
    fn test1() {
        let test_input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let mut data = Data::from_str(test_input);
        data.tilt(Direction::North);

        assert_eq!(136, data.get_load());

        data.tilt(Direction::North);

        let tilted_input = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

        let data2 = Data::from_str(tilted_input);

        assert_eq!(data.grid, data2.grid);

        let cycled_input = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";

        let data3 = Data::from_str(cycled_input);

        data.cycle();
        data.cycle();
        data.cycle();

        assert_eq!(data.grid, data3.grid);
    }

    #[test]
    fn tilt1() {
        let input = ".O.#..OO#O.O";
        let mut data = Data::from_str(input);

        data.tilt(Direction::East);

        assert_eq!(Data::from_str("..O#..OO#.OO").grid, data.grid);
    }

    #[test]
    fn test_range() {
        assert_eq!(range(3, 0).collect::<Vec<_>>(), vec![2, 1, 0]);
    }
}
