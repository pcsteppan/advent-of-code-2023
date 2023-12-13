use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum PipeType {
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Empty,
    Start,
}

impl PipeType {
    fn has_left(&self) -> bool {
        *self == PipeType::LeftRight || *self == PipeType::DownLeft || *self == PipeType::UpLeft
    }

    fn has_right(&self) -> bool {
        *self == PipeType::LeftRight || *self == PipeType::DownRight || *self == PipeType::UpRight
    }

    fn has_up(&self) -> bool {
        *self == PipeType::UpRight || *self == PipeType::UpLeft || *self == PipeType::UpDown
    }

    fn has_down(&self) -> bool {
        *self == PipeType::DownRight || *self == PipeType::DownLeft || *self == PipeType::UpDown
    }
}

#[derive(Debug, Eq, Hash, Clone)]
struct Node {
    position: (usize, usize),
    pipe_type: PipeType,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Vec<Node>>,
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct InternalState {
    count: usize,
    internal: bool,
    waiting_on: Option<PipeType>,
}

impl Graph {
    fn from_str(str: &str) -> Self {
        let nodes = str
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.chars()
                    .enumerate()
                    .map(|(col, c)| Node {
                        position: (row, col),
                        pipe_type: match c {
                            '|' => PipeType::UpDown,
                            '-' => PipeType::LeftRight,
                            'L' => PipeType::UpRight,
                            'J' => PipeType::UpLeft,
                            'F' => PipeType::DownRight,
                            '7' => PipeType::DownLeft,
                            'S' => PipeType::Start,
                            _ => PipeType::Empty,
                        },
                    })
                    .collect()
            })
            .collect();

        Graph { nodes }
    }

    fn get_all_distances(&self) -> HashMap<(usize, usize), usize> {
        let start_row = self
            .nodes
            .iter()
            .position(|row| row.iter().any(|node| node.pipe_type == PipeType::Start))
            .unwrap();
        let start_col = self.nodes[start_row]
            .iter()
            .position(|node| node.pipe_type == PipeType::Start)
            .unwrap();

        let start_node = &self.nodes[start_row][start_col];

        let mut visited: HashSet<&Node> = HashSet::new();
        let mut frontier = VecDeque::from([(start_node, 0)]);

        let mut distances = HashMap::new();
        distances.insert(start_node.position, 0);

        while !frontier.is_empty() {
            let (curr, distance) = frontier.pop_front().unwrap();

            if visited.contains(curr) {
                continue;
            }

            let left = (0, -1, Direction::Right);
            let right = (0, 1, Direction::Left);
            let up = (-1, 0, Direction::Down);
            let down = (1, 0, Direction::Up);

            let neighbors: Vec<(isize, isize, Direction)> = match curr.pipe_type {
                PipeType::Start => {
                    vec![up, down, left, right]
                }
                PipeType::DownLeft => {
                    vec![down, left]
                }
                PipeType::DownRight => {
                    vec![down, right]
                }
                PipeType::UpLeft => {
                    vec![up, left]
                }
                PipeType::UpRight => {
                    vec![up, right]
                }
                PipeType::UpDown => {
                    vec![up, down]
                }
                PipeType::LeftRight => {
                    vec![left, right]
                }
                PipeType::Empty => {
                    vec![]
                }
            };

            let graph_width = self.nodes.first().unwrap().len() as isize;
            let graph_height = self.nodes.len() as isize;

            let valid_neighbor_positions: Vec<_> = neighbors
                .iter()
                .map(|npos| {
                    (
                        npos.0 + curr.position.0 as isize,
                        npos.1 + curr.position.1 as isize,
                        npos.2,
                    )
                })
                .filter(|npos| {
                    let exists =
                        (0..graph_height).contains(&npos.0) && (0..graph_width).contains(&npos.1);
                    if !exists {
                        return false;
                    }

                    let n_pipe_type = self.nodes[npos.0 as usize][npos.1 as usize].pipe_type;

                    match npos.2 {
                        Direction::Left => n_pipe_type.has_left(),
                        Direction::Right => n_pipe_type.has_right(),
                        Direction::Up => n_pipe_type.has_up(),
                        Direction::Down => n_pipe_type.has_down(),
                    }
                })
                .collect();

            let valid_neighbors: Vec<_> = valid_neighbor_positions
                .into_iter()
                .map(|npos| (&self.nodes[npos.0 as usize][npos.1 as usize], distance + 1))
                .collect();

            for neighbor in valid_neighbors.as_slice() {
                distances
                    .entry(neighbor.0.position)
                    .and_modify(|entry| {
                        if *entry > neighbor.1 {
                            *entry = neighbor.1
                        }
                    })
                    .or_insert(neighbor.1);

                frontier.push_back(*neighbor);
            }

            visited.insert(curr);
        }
        distances
    }

    fn find_half_loop_length(&self) -> usize {
        let distances = self.get_all_distances();
        *distances.values().into_iter().max().unwrap()
    }

    fn get_clean_map(&self) -> Self {
        // clone self
        let mut new = self.clone();
        let distances = self.get_all_distances();
        let loop_node_positions: HashSet<(usize, usize)> =
            HashSet::from_iter(distances.into_keys().into_iter());

        for (row_i, row) in new.nodes.iter_mut().enumerate() {
            for (col_i, col) in row.iter_mut().enumerate() {
                if !loop_node_positions.contains(&(row_i, col_i)) {
                    col.pipe_type = PipeType::Empty;
                } else if col.pipe_type == PipeType::Start {
                    let left_connects =
                        col_i >= 1 && self.nodes[row_i][col_i - 1].pipe_type.has_right();
                    let right_connects = col_i < self.nodes[0].len() - 1
                        && self.nodes[row_i][col_i + 1].pipe_type.has_left();
                    let up_connects =
                        row_i >= 1 && self.nodes[row_i - 1][col_i].pipe_type.has_down();
                    let down_connects = row_i < self.nodes.len() - 1
                        && self.nodes[row_i + 1][col_i].pipe_type.has_up();

                    col.pipe_type =
                        match (up_connects, right_connects, down_connects, left_connects) {
                            (true, true, _, _) => PipeType::UpRight,
                            (true, _, true, _) => PipeType::UpDown,
                            (true, _, _, true) => PipeType::UpLeft,
                            (_, true, true, _) => PipeType::DownRight,
                            (_, true, _, true) => PipeType::LeftRight,
                            (_, _, true, true) => PipeType::DownLeft,
                            _ => panic!("Start node does not connect to two nodes"),
                        }
                }
            }
        }

        new
    }

    fn find_internal_space(&self) -> usize {
        let clean_map = self.get_clean_map();

        clean_map
            .nodes
            .iter()
            .map(|row| {
                row.iter()
                    .fold(
                        InternalState {
                            count: 0,
                            internal: false,
                            waiting_on: None,
                        },
                        |acc, curr| {
                            let mut is_next_internal = acc.internal;
                            if curr.pipe_type == PipeType::UpDown {
                                is_next_internal = !is_next_internal;
                            }
                            let next_is_waiting_on = if acc.waiting_on.is_some() {
                                if curr.pipe_type == acc.waiting_on.unwrap() {
                                    is_next_internal = !is_next_internal;
                                    None
                                } else if curr.pipe_type != PipeType::LeftRight {
                                    None
                                } else {
                                    acc.waiting_on
                                }
                            } else {
                                if curr.pipe_type == PipeType::UpRight {
                                    Some(PipeType::DownLeft)
                                } else if curr.pipe_type == PipeType::DownRight {
                                    Some(PipeType::UpLeft)
                                } else {
                                    None
                                }
                            };
                            InternalState {
                                count: acc.count
                                    + if acc.internal && curr.pipe_type == PipeType::Empty {
                                        1
                                    } else {
                                        0
                                    },
                                internal: is_next_internal,
                                waiting_on: next_is_waiting_on,
                            }
                        },
                    )
                    .count
            })
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not open input.txt");
    let graph = Graph::from_str(&input);

    let half_loop_length = graph.find_half_loop_length();
    println!("part 1: {}", half_loop_length);

    let internal_spaces = graph.find_internal_space();
    println!("part 2: {}", internal_spaces);
}

#[cfg(test)]
mod tests {
    use crate::Graph;

    #[test]
    fn test1() {
        let test_input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let graph = Graph::from_str(test_input);
        let half_loop_length = graph.find_half_loop_length();

        assert_eq!(8, half_loop_length)
    }

    #[test]
    fn test2() {
        let test_input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let graph = Graph::from_str(test_input);
        let half_loop_length = graph.find_half_loop_length();

        assert_eq!(4, half_loop_length)
    }

    #[test]
    fn test3() {
        let test_input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let graph = Graph::from_str(test_input);

        let internal_spaces = graph.find_internal_space();

        assert_eq!(10, internal_spaces);
    }
}
