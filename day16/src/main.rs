use std::{collections::HashSet, fs, i32};

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
struct Pos(i32, i32);

#[derive(Clone, Debug)]
enum Thing {
    Slash,
    Backslash,
    PipeSplitter,
    DashSplitter,
    Empty,
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Pos {
    fn next(&self, direction: Direction) -> Pos {
        match direction {
            Direction::North => Pos(self.0, self.1 - 1),
            Direction::East => Pos(self.0 + 1, self.1),
            Direction::South => Pos(self.0, self.1 + 1),
            Direction::West => Pos(self.0 - 1, self.1),
        }
    }
}

fn find_energized_cells(grid: &Vec<Vec<Thing>>) -> HashSet<Pos> {
    let mut visited = HashSet::new();

    let start = (Pos(-1, 0), Pos(0, 0));
    let mut frontier = vec![start];

    while let Some(light_ray) = frontier.pop() {
        if visited.contains(&light_ray) {
            continue;
        }

        let (prev, curr) = light_ray.clone();

        if !(curr.0 >= 0
            && curr.0 < grid[0].len() as i32
            && curr.1 >= 0
            && curr.1 < grid.len() as i32)
        {
            continue;
        }

        let from_direction = match (curr.0 - prev.0, curr.1 - prev.1) {
            (-1, 0) => Direction::East,
            (1, 0) => Direction::West,
            (0, -1) => Direction::South,
            (0, 1) => Direction::North,
            _ => panic!("unexpected from direction"),
        };

        let curr_thing = &grid[curr.1 as usize][curr.0 as usize];

        let mut next = match curr_thing {
            Thing::Empty => match from_direction {
                Direction::North => vec![curr.next(Direction::South)],
                Direction::East => vec![curr.next(Direction::West)],
                Direction::South => vec![curr.next(Direction::North)],
                Direction::West => vec![curr.next(Direction::East)],
            },
            Thing::Slash => match from_direction {
                Direction::North => vec![curr.next(Direction::West)],
                Direction::East => vec![curr.next(Direction::South)],
                Direction::South => vec![curr.next(Direction::East)],
                Direction::West => vec![curr.next(Direction::North)],
            },
            Thing::Backslash => match from_direction {
                Direction::North => vec![curr.next(Direction::East)],
                Direction::East => vec![curr.next(Direction::North)],
                Direction::South => vec![curr.next(Direction::West)],
                Direction::West => vec![curr.next(Direction::South)],
            },
            Thing::PipeSplitter => match from_direction {
                Direction::North => vec![curr.next(Direction::South)],
                Direction::East | Direction::West => {
                    vec![curr.next(Direction::North), curr.next(Direction::South)]
                }
                Direction::South => vec![curr.next(Direction::North)],
            },
            Thing::DashSplitter => match from_direction {
                Direction::North | Direction::South => {
                    vec![curr.next(Direction::East), curr.next(Direction::West)]
                }
                Direction::East => vec![curr.next(Direction::West)],
                Direction::West => vec![curr.next(Direction::East)],
            },
        }
        .iter()
        .map(|ray| (curr.clone(), ray.clone()))
        .collect::<Vec<_>>();

        visited.insert(light_ray);
        frontier.append(&mut next);
    }

    HashSet::from_iter(visited.iter().map(|i| i.1.clone()))
}

fn grid_from_str(str: &str) -> Vec<Vec<Thing>> {
    str.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '/' => Thing::Slash,
                    '\\' => Thing::Backslash,
                    '|' => Thing::PipeSplitter,
                    '-' => Thing::DashSplitter,
                    '.' => Thing::Empty,
                    _ => panic!("unexpected character: {}", c),
                })
                .collect()
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input.txt");
    let grid = grid_from_str(&input);
    let cells = find_energized_cells(&grid);
    println!("part 1: {}", cells.len());
}

#[cfg(test)]
mod test {
    use crate::{find_energized_cells, grid_from_str};

    #[test]
    fn test1() {
        let input = ".\\.
...
.-/";
        let grid = grid_from_str(&input);
        dbg!(grid.clone());
        let cells = find_energized_cells(&grid);
        assert_eq!(8, cells.len());
    }

    #[test]
    fn test2() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

        let grid = grid_from_str(&input);
        let cells = find_energized_cells(&grid);
        assert_eq!(46, cells.len());
    }
}
