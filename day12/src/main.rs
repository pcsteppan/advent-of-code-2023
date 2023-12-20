fn main() {
    println!("Hello, world!");
}

// #.#.### 1,1,3
#[derive(Debug)]
enum Thing {
    Broke,
    Gear,
    Maybe,
}

impl Thing {
    fn from_char(c: char) -> Thing {
        match c {
            '#' => Thing::Broke,
            '.' => Thing::Gear,
            '?' => Thing::Maybe,
            _ => panic!("unexpected char: {}", c),
        }
    }
}

#[derive(Debug)]
struct Diagram {
    state: Vec<Thing>,
    template: Vec<usize>,
}

impl Diagram {
    fn from_str(str: &str) -> Diagram {
        let (state_str, template_str) = str.split_once(" ").unwrap();

        Diagram {
            state: state_str.chars().map(Thing::from_char).collect(),
            template: template_str
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        }
    }

    fn is_possible(&self) -> bool {
        todo!();
    }

    fn find_regions(&self) -> Vec<usize> {}
}

fn fits(state: Diagram) {
    dbg!(state);
}

#[cfg(test)]
mod test {
    use crate::{State, Thing};

    #[test]
    fn test1() {
        //
    }
}
