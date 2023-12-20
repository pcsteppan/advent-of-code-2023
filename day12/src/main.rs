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
struct State {
    things: Vec<Thing>,
    template: Vec<usize>,
}

impl State {
    fn from_str(str: &str) -> State {
        //
    }
}

fn fits(state: State) {
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
