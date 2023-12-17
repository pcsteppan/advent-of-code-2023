use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not open input.txt");
    let instructions = input.trim().split(",");
    let result: u64 = instructions.clone().map(get_hash).sum();

    println!("part 1: {}", result);

    //let boxes: HashMap = HashMap::new();
    let mut boxes: Vec<HashMap<&str, (usize, u8)>> = (0..256).map(|_| HashMap::new()).collect();

    instructions.for_each(|instruction| {
        if let Some(idx) = instruction.find("-") {
            let key = &instruction[..idx];
            let hash = get_hash(&key);
            boxes[hash as usize].remove(&key);
        } else if instruction.find("=").is_some() {
            let (key, focal_length_str) = instruction.split_once("=").unwrap();
            let focal_length = focal_length_str.parse().unwrap();
            //dbg!(key, focal_length_str, focal_length);
            let hash = get_hash(key);
            let next_lens_slot = boxes[hash as usize]
                .values()
                .map(|l| l.0)
                .max()
                .unwrap_or(0)
                + 1;
            boxes[hash as usize]
                .entry(key)
                .and_modify(|old_f_length| *old_f_length = (old_f_length.0, focal_length))
                .or_insert((next_lens_slot, focal_length));
        }
    });

    let power: usize = boxes
        .iter()
        .enumerate()
        .map(|(i, lens_box)| {
            let mut vals: Vec<_> = lens_box.values().into_iter().collect();
            vals.sort_by_key(|v| v.0);
            vals.iter()
                .enumerate()
                .map(|(j, v)| (i + 1) * (j + 1) * v.1 as usize)
                .sum::<usize>()
        })
        .sum();

    println!("part 2: {}", power);
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
