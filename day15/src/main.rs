use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not open input.txt");
    let instructions = input.trim().split(",");
    let result: u64 = instructions.map(get_hash).sum();

    println!("part 1: {}", result);
}

fn get_hash(str: &str) -> u64 {
    str.chars()
        .map(|c| c as u64)
        .fold(0, |acc, curr| ((acc + curr) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use crate::get_hash;

    #[test]
    fn test1() {
        let input = "cm-";
        assert_eq!(get_hash(input), 253);
    }

    #[test]
    fn test2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let instructions = input.split(",");
        let result: u64 = instructions.map(get_hash).sum();

        assert_eq!(result, 1320);
    }
}
