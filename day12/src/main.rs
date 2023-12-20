use std::{collections::HashMap, fs, time::Instant};

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not open input.txt");
    let start = Instant::now();
    let result: usize = input
        .lines()
        .map(|l| Diagram::from_str(l, 1))
        .map(|d| d.find_all_valid_states())
        .sum();
    println!("part 1: {}, time elapsed: {:?}", result, start.elapsed());

    let p2_start = Instant::now();
    let p2_result: usize = input
        .lines()
        .map(|l| Diagram::from_str(l, 5))
        .map(|d| d.find_all_valid_states())
        .sum();
    println!(
        "part 1: {}, time elapsed: {:?}",
        p2_result,
        p2_start.elapsed()
    );
}

// #.#.### 1,1,3
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Thing {
    Empty,
    Gear,
    Maybe,
}

impl Thing {
    fn from_char(c: char) -> Thing {
        match c {
            '#' => Thing::Gear,
            '.' => Thing::Empty,
            '?' => Thing::Maybe,
            _ => panic!("unexpected char: {}", c),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Diagram {
    state: Vec<Thing>,
    template: Vec<usize>,
    template_total: usize,
    gear_count: usize,
    maybe_count: usize,
}

impl Diagram {
    fn from_str(str: &str, repeat: usize) -> Diagram {
        let (state_str, template_str) = str.split_once(" ").unwrap();
        let mut state_string = state_str.to_string();
        let mut template_string = template_str.to_string();

        if repeat > 1 {
            let mut repeated_state = vec![];
            let mut repeated_template = vec![];

            for _ in 0..repeat {
                repeated_state.push(state_str);
                repeated_template.push(template_str);
            }

            state_string = repeated_state.join("?");
            template_string = repeated_template.join(",");
        }

        let template: Vec<usize> = template_string
            .split(",")
            .map(|c| c.parse().unwrap())
            .collect();

        Diagram {
            state: state_string.chars().map(Thing::from_char).collect(),
            template_total: template.iter().sum(),
            template,
            gear_count: state_string.chars().filter(|c| *c == '#').count(),
            maybe_count: state_string.chars().filter(|c| *c == '?').count(),
        }
    }

    // heuristic function for culling branches with no potential
    fn is_possible(&self) -> bool {
        if self.gear_count > self.template_total
            || self.maybe_count + self.gear_count < self.template_total
        {
            return false;
        }

        return self.template.len() == 0
            || self.get_first_potential_region_size() >= self.template[0];
    }

    fn get_first_potential_region_size(&self) -> usize {
        let mut skip = false;
        let region_size =
            self.state
                .iter()
                .fold((0, (0, false)), |(max, (current_size, is_fixed)), value| {
                    if skip {
                        return (max, (current_size, is_fixed));
                    }
                    match value {
                        Thing::Empty => {
                            if current_size > 0 {
                                skip = is_fixed;
                                return (max.max(current_size), (0, false));
                            }

                            (max, (0, false))
                        }
                        Thing::Gear => (max, (current_size + 1, true)),
                        Thing::Maybe => (max, (current_size + 1, is_fixed)),
                    }
                });

        region_size.0.max(region_size.1 .0)
    }

    fn find_all_valid_states(&self) -> usize {
        self.get_number_of_valid_states(&mut HashMap::new())
    }

    fn get_number_of_valid_states(&self, memo: &mut HashMap<Diagram, usize>) -> usize {
        if let Some(memoized_count) = memo.get(&self) {
            return *memoized_count;
        }

        if !self.is_possible() {
            return 0;
        }

        if self.state.len() == 0 {
            return if self.template.len() == 0 { 1 } else { 0 };
        }

        if self.template.len() == 0 {
            return if self.state.iter().all(|t| *t != Thing::Gear) {
                1
            } else {
                0
            };
        }

        let mut results = 0;

        let head = self.state[0];
        if head == Thing::Empty || head == Thing::Maybe {
            let new = Diagram {
                state: self.state[1..].to_vec(),
                template: self.template.clone(),
                template_total: self.template_total,
                gear_count: self.gear_count,
                maybe_count: self.maybe_count - if head == Thing::Maybe { 1 } else { 0 },
            };

            results += new.get_number_of_valid_states(memo);
            if results == 1 {}
        }

        if (head == Thing::Gear || head == Thing::Maybe)
            && (self.template[0] <= self.state.len()
                && (self.template[0] == self.state.len()
                    || self.state[self.template[0]] != Thing::Gear)
                && self.state[..self.template[0]]
                    .iter()
                    .all(|t| *t != Thing::Empty))
        {
            let (gears_in_region, maybes_in_region) = self.state
                [..=(self.template[0].min(self.state.len() - 1))]
                .iter()
                .fold((0, 0), |acc, curr| match curr {
                    Thing::Gear => (acc.0 + 1, acc.1),
                    Thing::Maybe => (acc.0, acc.1 + 1),
                    _ => acc,
                });

            let new = Diagram {
                state: self.state[(self.template[0] + 1).min(self.state.len())..].to_vec(),
                template: self.template[1..].to_vec(),
                template_total: self.template_total - self.template[0],
                gear_count: self.gear_count - gears_in_region,
                maybe_count: self.maybe_count - maybes_in_region,
            };

            results += new.get_number_of_valid_states(memo);
        }

        memo.insert(self.clone(), results);

        results
    }
}

#[cfg(test)]
mod test {
    use crate::Diagram;

    #[test]
    fn test1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let diagrams = input.lines().map(|l| Diagram::from_str(l, 1));

        let expected_results = [1, 4, 1, 1, 4, 10];
        diagrams
            .zip(expected_results)
            .for_each(|(diagram, expected_result)| {
                assert_eq!(expected_result, diagram.find_all_valid_states())
            });
    }
}
