/// LeetCode #585 - Investments in 2016 (SQL; Rust analogue)

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

use std::collections::HashMap;

fn investments_in_2016(insurance: Vec<(i32, f64, f64, i32, i32)>) -> f64 {
    let mut tiv: HashMap<i64, i32> = HashMap::new();
    let mut loc: HashMap<(i32, i32), i32> = HashMap::new();
    for (_, t2015, _, lat, lon) in &insurance {
        let key = (*t2015 * 1000.0).round() as i64;
        *tiv.entry(key).or_insert(0) += 1;
        *loc.entry((*lat, *lon)).or_insert(0) += 1;
    }
    let mut sum = 0.0;
    for (_, t2015, t2016, lat, lon) in &insurance {
        let key = (*t2015 * 1000.0).round() as i64;
        if *tiv.get(&key).unwrap_or(&0) > 1 && *loc.get(&(*lat, *lon)).unwrap_or(&0) == 1 {
            sum += *t2016;
        }
    }
    round2(sum)
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::investments_in_2016;

    #[test]
    fn example() {
        let insurance = vec![
            (1, 10.0, 5.0, 10, 10),
            (2, 20.0, 20.0, 20, 20),
            (3, 10.0, 30.0, 20, 20),
            (4, 10.0, 40.0, 40, 40),
        ];
        assert!((investments_in_2016(insurance) - 45.0).abs() < 1e-9);
    }
}
