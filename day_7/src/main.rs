use std::collections::HashMap;

struct Network {
    nodes: HashMap<String, (String, String)>,
    instructions: Vec<bool>,
}

impl Network {
    fn from_str(str: &str) -> Network {
        let mut lines = str.lines();
        let instructions = lines.next().unwrap().chars().map(|c| if c == 'L' {true} else {false}).collect();
        let nodes = HashMap::new();

        Network {
            nodes,
            instructions
        }
    }
}

fn main() {
    
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
    }
}
