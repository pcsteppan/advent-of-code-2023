fn quadratic_roots(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    Some((
        (-b + discriminant.sqrt()) / (2.0 * a),
        (-b - discriminant.sqrt()) / (2.0 * a),
    ))
}

fn get_winning_count(duration: i64, threshold: i64) -> i64 {
    let roots = quadratic_roots(-1.0, (duration as f64) - 0.001, -1.0 * threshold as f64).unwrap();
    (roots.1.floor() - roots.0.ceil()) as i64 + 1
}

fn main() {
    let result = vec![(7, 9), (15, 40), (30, 200)]
        .into_iter()
        .map(|(b, c)| get_winning_count(b, c))
        .fold(1, |acc, curr| acc * curr);

    // part 1
    println!("part 1: {}", result);

    // part 2
    println!("part 2: {}", get_winning_count(56977875, 546192711311139));
}
