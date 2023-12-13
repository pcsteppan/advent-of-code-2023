use std::fs;

struct GalaxyMap {
    map: Vec<Vec<bool>>,
}

impl GalaxyMap {
    fn from_str(str: &str) -> Self {
        GalaxyMap {
            map: str
                .lines()
                .into_iter()
                .map(|l| l.chars().into_iter().map(|c| c == '#').collect())
                .collect(),
        }
    }

    fn compress_map(&self, map: &Vec<Vec<bool>>) -> Vec<usize> {
        let mut compressed_map = vec![];
        (0..map[0].len()).into_iter().for_each(|i| {
            let col_count = (0..map.len())
                .into_iter()
                .fold(0, |acc, curr| acc + if map[curr][i] { 1 } else { 0 });
            if col_count > 0 {
                compressed_map.push(col_count);
            } else {
                compressed_map.push(0);
                compressed_map.push(0);
            }
        });

        compressed_map
    }

    fn find_horizontal_distances(&self, map: &Vec<Vec<bool>>) -> usize {
        let compressed_map = self.compress_map(map);

        let mut total = 0;

        (0..compressed_map.len()).for_each(|i| {
            let galaxies_at_i = compressed_map[i];

            if galaxies_at_i > 0 {
                (i + 1..compressed_map.len()).for_each(|j| {
                    let galaxies_at_j = compressed_map[j];

                    let distances_for_galaxies_at_i_and_j = galaxies_at_i * galaxies_at_j * (j - i);

                    println!("{} {} {}", i, j, distances_for_galaxies_at_i_and_j);

                    total += distances_for_galaxies_at_i_and_j;
                })
            }
        });

        total
    }

    fn find_distances(&self) -> usize {
        let horizontal_distances = self.find_horizontal_distances(&self.map);
        let vertical_distances = self.find_horizontal_distances(&transpose(self.map.clone()));
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
    let distances = galaxy_map.find_distances();

    println!("part 1: {}", distances);
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

        assert_eq!(374, galaxy_map.find_distances());
    }

    #[test]
    fn test2() {
        let input = "#...
....
#.##";
        let galaxy_map = GalaxyMap::from_str(input);

        assert_eq!(24, galaxy_map.find_distances());
    }
}
