/// LeetCode #3793 - Find Users with High Token Usage (SQL; Rust analogue)
use std::collections::HashMap;

/// row: (user_id, prompt, tokens)
fn find_users_with_high_tokens(prompts: Vec<(i32, String, i32)>) -> Vec<(i32, i32, f64)> {
    struct Agg {
        count: i32,
        sum: i64,
        max: i32,
    }
    let mut map: HashMap<i32, Agg> = HashMap::new();
    for (uid, _p, tokens) in prompts {
        let e = map.entry(uid).or_insert(Agg {
            count: 0,
            sum: 0,
            max: i32::MIN,
        });
        e.count += 1;
        e.sum += tokens as i64;
        e.max = e.max.max(tokens);
    }
    let mut ans: Vec<(i32, i32, f64)> = map
        .into_iter()
        .filter_map(|(uid, a)| {
            if a.count < 3 {
                return None;
            }
            let avg = (a.sum as f64 / a.count as f64 * 100.0).round() / 100.0;
            if (a.max as f64) <= avg {
                return None;
            }
            Some((uid, a.count, avg))
        })
        .collect();
    ans.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap().then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    println!("{:?}", find_users_with_high_tokens(vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_users_with_high_tokens;

    #[test]
    fn example() {
        let rows = vec![
            (1, "Write a blog outline".into(), 120),
            (1, "Generate SQL query".into(), 80),
            (1, "Summarize an article".into(), 200),
            (2, "Create resume bullet".into(), 60),
            (2, "Improve LinkedIn bio".into(), 70),
            (3, "Explain neural networks".into(), 300),
            (3, "Generate interview Q&A".into(), 250),
            (3, "Write cover letter".into(), 180),
            (3, "Optimize Python code".into(), 220),
        ];
        let ans = find_users_with_high_tokens(rows);
        assert_eq!(ans.len(), 2);
        assert_eq!(ans[0].0, 3);
        assert_eq!(ans[0].1, 4);
        assert!((ans[0].2 - 237.5).abs() < 1e-9);
        assert_eq!(ans[1].0, 1);
        assert_eq!(ans[1].1, 3);
        assert!((ans[1].2 - 133.33).abs() < 1e-9);
    }
}
