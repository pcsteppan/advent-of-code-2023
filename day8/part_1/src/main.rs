use std::{collections::HashMap, fs};

struct Network {
    nodes: HashMap<String, (String, String)>,
    instructions: Vec<bool>,
}

impl Network {
    fn from_str(str: &str) -> Network {
        let mut lines = str.lines();
        let instructions = lines.next().unwrap().chars().map(|c| if c == 'L' {true} else {false}).collect();
        lines.next();
        let proto_nodes = lines.into_iter().map(|l| {
            let (key, values) = l.split_once(" = ").unwrap();
            let (value1, value2) = values.split_once(", ").unwrap();
            (key.trim().to_string(), (value1[1..].to_string(), value2[..value2.len() - 1].to_string()))
        });

        let nodes = HashMap::from_iter(proto_nodes);

        Network {
            nodes,
            instructions
        }
    }

    fn get_steps_to_zzz(&self) -> usize {
        let mut steps = 0;
        let mut curr = "AAA".to_string();

        while curr != "ZZZ" {
            let curr_instruction = self.instructions[steps % self.instructions.len()];
            // dbg!(self.nodes.get(&curr));
            curr = self.nodes.get(&curr).and_then(|t| if curr_instruction {Some(t.0.clone())} else {Some(t.1.clone())}).unwrap(); 
            steps += 1;
             
        }

        steps
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
        let test_input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let network = Network::from_str(test_input);
        let count = network.get_steps_to_zzz();

        assert_eq!(count, 6);
    }
}
