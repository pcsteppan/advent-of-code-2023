// Which garden plots can be reached with remaining steps?
// S = start, . = garden plots, # = rocks
//
// Starting at S, you can move in cardinal directions
//
// Plan is to find plot count at each step distance
// and then sum the plot counts at all even step distances

use std::collections::HashMap;

#[derive(Debug)]
struct Garden {
    plots: Vec<Vec<bool>>,
    start: (usize, usize),
}

impl Garden {
    fn from_str(str: &str) -> Self {
        let mut start = (0, 0);

        let plots = str
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        'S' => {
                            start = (start.0, start.1);
                            false
                        }
                        _ => panic!("Invalid character"),
                    })
                    .collect()
            })
            .collect();

        Self { plots, start }
    }
}

fn solve(input: &str, steps: usize) -> usize {
    let garden = Garden::from_str(input);

    let frontier = vec![garden.start];
    let map = [

    dbg!(garden);

    0
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        let steps = 6;

        let plot_count = 16;

        assert_eq!(solve(input, steps), plot_count);
    }
}
