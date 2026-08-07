/// LeetCode #3055 - Top Percentile Fraud (SQL; Rust analogue)
use std::collections::HashMap;

fn top_percentile_fraud(fraud: Vec<(i32, String, f64)>) -> Vec<(i32, String, f64)> {
    let mut max_by_state: HashMap<String, f64> = HashMap::new();

    for (_, state, score) in &fraud {
        max_by_state
            .entry(state.clone())
            .and_modify(|m| {
                if *score > *m {
                    *m = *score;
                }
            })
            .or_insert(*score);
    }

    let mut ans: Vec<_> = fraud
        .into_iter()
        .filter(|(_, state, score)| (score - max_by_state[state]).abs() < f64::EPSILON)
        .collect();
    ans.sort_by(|a, b| {
        a.1.cmp(&b.1)
            .then(b.2.partial_cmp(&a.2).unwrap())
            .then(a.0.cmp(&b.0))
    });
    ans
}

fn example_fraud() -> Vec<(i32, String, f64)> {
    vec![
        (1, "California".into(), 0.92),
        (2, "California".into(), 0.68),
        (3, "California".into(), 0.17),
        (4, "New York".into(), 0.94),
        (5, "New York".into(), 0.81),
        (6, "New York".into(), 0.77),
        (7, "Texas".into(), 0.98),
        (8, "Texas".into(), 0.97),
        (9, "Texas".into(), 0.96),
        (10, "Florida".into(), 0.97),
        (11, "Florida".into(), 0.98),
        (12, "Florida".into(), 0.78),
        (13, "Florida".into(), 0.88),
        (14, "Florida".into(), 0.66),
    ]
}

fn main() {
    println!("{:?}", top_percentile_fraud(example_fraud()));
}

#[cfg(test)]
mod tests {
    use super::{example_fraud, top_percentile_fraud};

    #[test]
    fn example() {
        assert_eq!(
            top_percentile_fraud(example_fraud()),
            vec![
                (1, "California".into(), 0.92),
                (11, "Florida".into(), 0.98),
                (4, "New York".into(), 0.94),
                (7, "Texas".into(), 0.98),
            ]
        );
    }
}
