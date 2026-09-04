/// LeetCode #578 - Get Highest Answer Rate Question (SQL; Rust analogue)
use std::collections::HashMap;

fn highest_answer_rate(
    survey_log: Vec<(i32, String, i32, Option<i32>, i32, i32)>,
) -> i32 {
    let mut show: HashMap<i32, i32> = HashMap::new();
    let mut answer: HashMap<i32, i32> = HashMap::new();
    for (_, action, qid, _, _, _) in survey_log {
        match action.as_str() {
            "show" => *show.entry(qid).or_insert(0) += 1,
            "answer" => *answer.entry(qid).or_insert(0) += 1,
            _ => {}
        }
    }
    let mut best_q = i32::MAX;
    let mut best_rate = -1.0f64;
    for (&qid, &s) in &show {
        let a = *answer.get(&qid).unwrap_or(&0);
        let rate = if s == 0 { 0.0 } else { a as f64 / s as f64 };
        if rate > best_rate + 1e-12 || ((rate - best_rate).abs() < 1e-12 && qid < best_q) {
            best_rate = rate;
            best_q = qid;
        }
    }
    best_q
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::highest_answer_rate;

    #[test]
    fn example() {
        let log = vec![
            (5, "show".into(), 285, None, 1, 123),
            (5, "answer".into(), 285, Some(124124), 1, 124),
            (5, "show".into(), 369, None, 2, 125),
            (5, "skip".into(), 369, None, 2, 126),
        ];
        assert_eq!(highest_answer_rate(log), 285);
    }
}
