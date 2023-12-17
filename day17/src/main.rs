use std::fs;

use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32, [u8; 4]);

impl Pos {
    fn successors(
        &self,
        data: &Vec<Vec<usize>>,
        min_distance: u8,
        max_distance: u8,
    ) -> Vec<(Pos, usize)> {
        let &Pos(x, y, directions) = self;

        let current_segment_distance = directions.iter().max().unwrap();
        let force_single_direction =
            current_segment_distance > &0 && current_segment_distance < &min_distance;

        let [n, e, s, w] = directions;
        let next = if force_single_direction {
            vec![match (n, e, s, w) {
                (_, 0, 0, 0) => Pos(x, y - 1, [n + 1, 0, 0, 0]),
                (0, _, 0, 0) => Pos(x + 1, y, [0, e + 1, 0, 0]),
                (0, 0, _, 0) => Pos(x, y + 1, [0, 0, s + 1, 0]),
                (0, 0, 0, _) => Pos(x - 1, y, [0, 0, 0, w + 1]),
                _ => panic!("unexpected condition"),
            }]
        } else {
            vec![
                Pos(x, y - 1, [n + 1, 0, 0, 0]),
                Pos(x + 1, y, [0, e + 1, 0, 0]),
                Pos(x, y + 1, [0, 0, s + 1, 0]),
                Pos(x - 1, y, [0, 0, 0, w + 1]),
            ]
        };

        next.into_iter()
            .filter(|p| {
                p.0 >= 0 && p.0 < data[0].len() as i32 && p.1 >= 0 && p.1 < data.len() as i32
            })
            .filter(|p| {
                // Only turn LEFT or RIGHT (no reversing)
                p.2[0] > 0 && self.2[2] == 0
                    || p.2[1] > 0 && self.2[3] == 0
                    || p.2[2] > 0 && self.2[0] == 0
                    || p.2[3] > 0 && self.2[1] == 0
            })
            .map(|p| {
                (
                    p.clone(),
                    if p.2.iter().any(|v| *v > max_distance) {
                        10000
                    } else {
                        data[p.1 as usize][p.0 as usize]
                    },
                )
            })
            .collect()
    }
}

fn grid_from_str(str: &str) -> Vec<Vec<usize>> {
    str.lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn find_best_path_result(
    data: &Vec<Vec<usize>>,
    min_distance: u8,
    max_distance: u8,
) -> Option<(Vec<Pos>, usize)> {
    let goal = Pos(
        data[0].len() as i32 - 1,
        data.len() as i32 - 1,
        [0, 0, 0, 0],
    );
    let result = dijkstra(
        &Pos(0, 0, [0, 0, 0, 0]),
        |p| p.successors(data, min_distance, max_distance),
        |p| p.0 == goal.0 && p.1 == goal.1 && p.2.iter().max().unwrap() >= &min_distance,
    );
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input.txt");
    let data = grid_from_str(&input);
    let result = find_best_path_result(&data, 0, 3);

    println!("part 1: {:?}", result.unwrap().1);

    let result2 = find_best_path_result(&data, 4, 10);

    println!("part 2: {:?}", result2.unwrap().1);
}

#[cfg(test)]
mod test {
    use crate::{find_best_path_result, grid_from_str};

    #[test]
    fn test1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let result = find_best_path_result(&grid_from_str(&input), 0, 3);

        assert_eq!(102, result.unwrap().1);

        let result2 = find_best_path_result(&grid_from_str(&input), 4, 10);

        assert_eq!(94, result2.unwrap().1);
    }

    #[test]
    fn test2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";

        let result = find_best_path_result(&grid_from_str(&input), 4, 10);
        dbg!(result.clone());
        assert_eq!(71, result.unwrap().1);
    }
}
