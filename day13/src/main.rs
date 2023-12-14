use std::{collections::hash_map::DefaultHasher, fs, hash::BuildHasher};

#[derive(PartialEq, Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct Terrain {
    grid: Vec<Vec<bool>>,
}

impl Terrain {
    fn from_str(str: &str) -> Terrain {
        Terrain {
            grid: str
                .lines()
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect(),
        }
    }

    fn get_row_hashes(&self) -> Vec<u64> {
        let b = std::collections::hash_map::RandomState::new();
        self.grid.iter().map(|row| b.hash_one(row)).collect()
    }

    fn get_col_hashes(&self) -> Vec<u64> {
        let transposed_terrain = Terrain {
            grid: (0..self.grid[0].len())
                .map(|i| self.grid.iter().map(|row| row[i]).collect())
                .collect(),
        };

        transposed_terrain.get_row_hashes()
    }

    fn get_point_of_symmetry(&self, orientation: Orientation) -> Option<usize> {
        let vec = if orientation == Orientation::Horizontal {
            self.get_row_hashes()
        } else {
            self.get_col_hashes()
        };

        (0..vec.len() - 1)
            .map(|i| (i, Self::find_symmetry_at_index(&vec, i as isize, i + 1)))
            .filter(|result| result.1 == (result.0 + 1).min(vec.len() - result.0 - 1))
            .max_by_key(|result| result.1)
            .map(|result| result.0 + 1)
    }

    fn get_points_of_symmetry(&self) -> [(usize, Orientation); 2] {
        [
            (
                self.get_point_of_symmetry(Orientation::Horizontal)
                    .unwrap_or(0),
                Orientation::Horizontal,
            ),
            (
                self.get_point_of_symmetry(Orientation::Vertical)
                    .unwrap_or(0),
                Orientation::Vertical,
            ),
        ]
    }

    fn find_symmetry_at_index(vec: &Vec<u64>, i: isize, j: usize) -> usize {
        if i < 0 || j == vec.len() || vec[i as usize] != vec[j] {
            return 0;
        }

        1 + Self::find_symmetry_at_index(vec, i - 1, j + 1)
    }

    fn find_summary(&self) -> usize {
        let results = self.get_points_of_symmetry();

        results
            .iter()
            .map(|result| {
                result.0
                    * if result.1 == Orientation::Vertical {
                        1
                    } else {
                        100
                    }
            })
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input.txt");
    let terrains: Vec<_> = input.split("\r\n\r\n").map(Terrain::from_str).collect();

    let part1_answer: usize = terrains
        .iter()
        .map(|terrain| {
            let result = terrain.find_summary();
            assert!(result != 0);

            result
        })
        .sum();

    println!("part 1: {}", part1_answer);
}

#[cfg(test)]
mod tests {
    use crate::{Orientation, Terrain};

    #[test]
    fn test1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let terrain = Terrain::from_str(input);

        assert_eq!(None, terrain.get_point_of_symmetry(Orientation::Horizontal));
        assert_eq!(
            Some(5),
            terrain.get_point_of_symmetry(Orientation::Vertical)
        );
        assert_eq!(5, terrain.find_summary());
    }

    #[test]
    fn test2() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let terrain = Terrain::from_str(input);

        assert_eq!(400, terrain.find_summary());
    }

    #[test]
    fn test3() {
        let input = ".##.";
        let terrain = Terrain::from_str(input);

        assert_eq!(
            Some(2),
            terrain.get_point_of_symmetry(Orientation::Vertical)
        );
    }

    #[test]
    fn test4() {
        let input = "..#..#.###..#
.###.#...#..#
..#.###.#####
.##..##.#.##.
.##..##.#.##.
..#.###.#####
.###.....#..#
..#..#.###..#
.#..##....##.
##.##.#...##.
#.##.#...####";

        let terrain = Terrain::from_str(input);

        assert_eq!(11, terrain.find_summary());
    }
}
