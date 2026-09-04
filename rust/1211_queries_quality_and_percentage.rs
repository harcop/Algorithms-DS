/// LeetCode #1211 - Queries Quality and Percentage (SQL; Rust analogue)

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

use std::collections::HashMap;

fn queries_quality(queries: Vec<(String, String, i32, i32)>) -> Vec<(String, f64, f64)> {
    let mut acc: HashMap<String, (f64, i32, i32)> = HashMap::new();
    for (name, _, position, rating) in queries {
        let e = acc.entry(name).or_insert((0.0, 0, 0));
        e.0 += rating as f64 / position as f64;
        e.1 += 1;
        if rating < 3 {
            e.2 += 1;
        }
    }
    let mut ans: Vec<(String, f64, f64)> = acc
        .into_iter()
        .map(|(n, (s, c, poor))| (n, round2(s / c as f64), round2(poor as f64 / c as f64 * 100.0)))
        .collect();
    ans.sort_by(|a, b| b.0.cmp(&a.0));
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::queries_quality;

    #[test]
    fn example() {
        let queries = vec![
            ("Dog".into(), "Golden Retriever".into(), 1, 5),
            ("Dog".into(), "German Shepherd".into(), 2, 5),
            ("Dog".into(), "Mule".into(), 200, 1),
            ("Cat".into(), "Shirazi".into(), 5, 2),
            ("Cat".into(), "Siamese".into(), 3, 3),
            ("Cat".into(), "Sphynx".into(), 7, 4),
        ];
        let got = queries_quality(queries);
        assert_eq!(got[0].0, "Dog");
        assert!((got[0].1 - 2.50).abs() < 1e-9);
        assert!((got[0].2 - 33.33).abs() < 1e-9);
        assert_eq!(got[1].0, "Cat");
        assert!((got[1].1 - 0.66).abs() < 1e-9);
        assert!((got[1].2 - 33.33).abs() < 1e-9);
    }
}
