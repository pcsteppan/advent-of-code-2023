use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not find input.txt");

    let sum: isize = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| find_next_element(v))
        .sum();

    println!("part 1: {}", sum);

    let sum2: isize = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| find_prev_element(v))
        .sum();

    println!("part 2: {}", sum2);
}

fn find_prev_element(list: Vec<isize>) -> isize {
    let mut rev_list = list.clone();
    rev_list.reverse();
    find_next_element(rev_list)
}

fn find_next_element(list: Vec<isize>) -> isize {
    if list.iter().all(|n| n == &0) {
        return 0;
    };

    let diff_list: Vec<_> = list.windows(2).into_iter().map(|w| w[1] - w[0]).collect();

    return list.last().unwrap() + find_next_element(diff_list);
}

#[cfg(test)]
mod tests {
    use crate::{find_next_element, find_prev_element};

    #[test]
    fn test1() {
        let result = find_next_element(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(18, result);
    }

    #[test]
    fn test2() {
        let result = find_next_element(vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(result, 28);
    }

    #[test]
    fn test3() {
        let result = find_prev_element(vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(result, 5);
    }
}
