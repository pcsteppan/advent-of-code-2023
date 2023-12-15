use std::fs;

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
    fn get_transpose(&self) -> Self {
        Terrain {
            grid: (0..self.grid[0].len())
                .map(|i| self.grid.iter().map(|row| row[i]).collect())
                .collect(),
        }
    }

    fn get_point_of_symmetry_wrapper(&self, orientation: Orientation) -> Option<usize> {
        if orientation == Orientation::Horizontal {
            return self.get_point_of_symmetry();
        }

        let transposed_self = self.get_transpose();
        transposed_self.get_point_of_symmetry()
    }

    fn get_point_of_symmetry(&self) -> Option<usize> {
        (0..self.grid.len() - 1)
            .map(|i| (i, Self::find_symmetry_at_index(self, i as isize, i + 1)))
            .filter(|result| {
                result.1 .1 == 1
                    && result.1 .0 == (result.0 + 1).min(self.grid.len() - result.0 - 1)
            })
            .max_by_key(|result| result.1)
            .map(|result| result.0 + 1)
    }

    fn get_points_of_symmetry(&self) -> [(usize, Orientation); 2] {
        [
            (
                self.get_point_of_symmetry_wrapper(Orientation::Horizontal)
                    .unwrap_or(0),
                Orientation::Horizontal,
            ),
            (
                self.get_point_of_symmetry_wrapper(Orientation::Vertical)
                    .unwrap_or(0),
                Orientation::Vertical,
            ),
        ]
    }

    // returns a tuple. the first item represents the number of symmetries.
    // the second item represents the number of 'smudges' in the symmetry.
    fn find_symmetry_at_index(&self, i: isize, j: usize) -> (usize, usize) {
        if i < 0 || j == self.grid.len() {
            return (0, 0);
        }

        let smudges = self.grid[i as usize]
            .iter()
            .zip(self.grid[j].iter())
            .filter(|(item_i, item_j)| item_i != item_j)
            .count();

        if smudges > 1 {
            return (0, 0);
        }

        let next = Self::find_symmetry_at_index(self, i - 1, j + 1);
        (1 + next.0, if smudges == 1 { 1 } else { 0 } + next.1)
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
