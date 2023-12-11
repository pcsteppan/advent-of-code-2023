use num_integer::lcm;
use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct Node {
    next: (String, String),
    cycle_record: HashMap<usize, (usize, usize)>,
}

impl Node {
    fn new(next: (String, String)) -> Node {
        Node {
            next,
            cycle_record: HashMap::new(),
        }
    }
}

struct Network {
    nodes: HashMap<String, Node>,
    instructions: Vec<bool>,
}

impl Network {
    fn from_str(str: &str) -> Network {
        let mut lines = str.lines();
        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| if c == 'L' { true } else { false })
            .collect();
        lines.next();
        let proto_nodes = lines.into_iter().map(|l| {
            let (key, values) = l.split_once(" = ").unwrap();
            let (value1, value2) = values.split_once(", ").unwrap();
            (
                key.trim().to_string(),
                Node::new((
                    value1[1..].to_string(),
                    value2[..value2.len() - 1].to_string(),
                )),
            )
        });

        let nodes = HashMap::from_iter(proto_nodes);

        Network {
            nodes,
            instructions,
        }
    }

    fn get_steps_to_zzz(mut self) -> usize {
        let mut steps = 0;

        // the set of nodes which end with A, which are iteratively updated by the instruction
        // command in parallel
        let mut curr_nodes: Vec<String> = self
            .nodes
            .keys()
            .into_iter()
            .filter(|key| key.ends_with('A'))
            .map(|key| key.clone())
            .collect();

        let end_nodes: Vec<String> = self
            .nodes
            .keys()
            .into_iter()
            .filter(|key| key.ends_with('Z'))
            .map(|key| key.clone())
            .collect();

        while !end_nodes
            .iter()
            .map(|n| self.nodes.get(n).unwrap())
            .all(|n| {
                n.cycle_record
                    .get(&(steps % self.instructions.len()))
                    .unwrap_or(&(0, 0))
                    .1
                    > 0
            })
        {
            let ci_index = steps % self.instructions.len();
            let curr_instruction = self.instructions[ci_index];

            self.nodes
                .iter_mut()
                .filter(|(k, _)| end_nodes.contains(k) && curr_nodes.contains(k))
                .for_each(|(_, v)| {
                    v.cycle_record
                        .entry(ci_index)
                        .and_modify(|record| {
                            if record.1 == 0 {
                                record.1 = steps
                            }
                        })
                        .or_insert((steps, 0));
                });

            curr_nodes = curr_nodes
                .into_iter()
                .map(|node| {
                    self.nodes
                        .get(&node)
                        .and_then(|t| {
                            if curr_instruction {
                                Some(t.next.0.clone())
                            } else {
                                Some(t.next.1.clone())
                            }
                        })
                        .unwrap()
                })
                .collect();

            steps += 1;
        }

        let end_node_values: Vec<_> = end_nodes
            .iter()
            .map(|n| self.nodes.get(n).unwrap())
            .collect();

        let i = (0..self.instructions.len())
            .into_iter()
            .find(|i| {
                end_node_values
                    .iter()
                    .all(|v| v.cycle_record.get(i).unwrap().1 > 0)
            })
            .unwrap();

        let lcm = end_node_values
            .iter()
            .map(|v| {
                let step_record = v.cycle_record.get(&i).unwrap();
                step_record.1 - step_record.0
            })
            .reduce(|acc, curr| lcm(acc, curr))
            .unwrap();

        lcm
    }
}

fn main() {
    let input = fs::read_to_string("../input.txt").expect("could not read input file");
    let network = Network::from_str(&input);

    dbg!(network.get_steps_to_zzz());
}

#[cfg(test)]
mod tests {
    use crate::Network;

    #[test]
    fn network() {
        let test_input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

        let network = Network::from_str(test_input);
        let count = network.get_steps_to_zzz();

        assert_eq!(count, 6);
    }
}
