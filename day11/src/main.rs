use std::{fs, time::Instant};

struct GalaxyMap {
    map: Vec<Vec<bool>>,
}

impl GalaxyMap {
    fn from_str(str: &str) -> Self {
        GalaxyMap {
            map: str
                .lines()
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect(),
        }
    }

    fn compress_map(&self, map: &Vec<Vec<bool>>) -> Vec<Option<usize>> {
        let mut compressed_map = vec![];
        (0..map[0].len()).for_each(|i| {
            let col_count =
                (0..map.len()).fold(0, |acc, curr| acc + if map[curr][i] { 1 } else { 0 });
            if col_count > 0 {
                compressed_map.push(Some(col_count));
            } else {
                compressed_map.push(None);
            }
        });

        compressed_map
    }

    fn find_horizontal_distances(&self, map: &Vec<Vec<bool>>, expansion_coeff: usize) -> usize {
        let compressed_map = self.compress_map(map);

        let mut total = 0;

        (0..compressed_map.len()).for_each(|i| {
            let galaxies_at_i = compressed_map[i];

            if galaxies_at_i.is_some() {
                let mut extra_space = 0;

                (i + 1..compressed_map.len()).for_each(|j| {
                    let galaxies_at_j = compressed_map[j];
                    if galaxies_at_j.is_none() {
                        extra_space += expansion_coeff - 1;
                    } else {
                        let distances_between_galaxies_at_i_and_j =
                            galaxies_at_i.unwrap() * galaxies_at_j.unwrap() * (j - i + extra_space);
                        total += distances_between_galaxies_at_i_and_j;
                    }
                })
            }
        });

        total
    }

    fn find_distances(&self, expansion_coeff: usize) -> usize {
        let horizontal_distances = self.find_horizontal_distances(&self.map, expansion_coeff);
        let vertical_distances =
            self.find_horizontal_distances(&transpose(self.map.clone()), expansion_coeff);
        horizontal_distances + vertical_distances
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input.txt");
    let galaxy_map = GalaxyMap::from_str(&input);

    let p1 = Instant::now();
    let distances = galaxy_map.find_distances(2);
    println!("part 1: {}, {:?}", distances, p1.elapsed());

    let p2 = Instant::now();
    let part2_distances = galaxy_map.find_distances(1_000_000);
    println!("part 2: {}, {:?}", part2_distances, p2.elapsed());
}

#[cfg(test)]
mod tests {
    use crate::GalaxyMap;

    #[test]
    fn test1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let galaxy_map = GalaxyMap::from_str(input);

        assert_eq!(374, galaxy_map.find_distances(2));
    }

    #[test]
    fn test2() {
        let input = "#...
....
#.##";
        let galaxy_map = GalaxyMap::from_str(input);

        assert_eq!(24, galaxy_map.find_distances(2));
    }
}
