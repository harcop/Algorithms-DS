/// LeetCode #2512 - Reward Top K Students
use std::collections::HashSet;

fn top_students(
    positive_feedback: Vec<String>,
    negative_feedback: Vec<String>,
    report: Vec<String>,
    student_id: Vec<i32>,
    k: i32,
) -> Vec<i32> {
    let ps: HashSet<&str> = positive_feedback.iter().map(|s| s.as_str()).collect();
    let ns: HashSet<&str> = negative_feedback.iter().map(|s| s.as_str()).collect();
    let mut arr = Vec::with_capacity(report.len());
    for (sid, r) in student_id.into_iter().zip(report.into_iter()) {
        let mut t = 0;
        for w in r.split_whitespace() {
            if ps.contains(w) {
                t += 3;
            } else if ns.contains(w) {
                t -= 1;
            }
        }
        arr.push((t, sid));
    }
    arr.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    arr.into_iter().take(k as usize).map(|(_, sid)| sid).collect()
}

fn main() {
    println!(
        "{:?}",
        top_students(
            vec![
                "smart".to_string(),
                "brilliant".to_string(),
                "studious".to_string()
            ],
            vec!["not".to_string()],
            vec![
                "this student is studious".to_string(),
                "the student is smart".to_string()
            ],
            vec![1, 2],
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::top_students;

    #[test]
    fn example_one() {
        assert_eq!(
            top_students(
                vec![
                    "smart".to_string(),
                    "brilliant".to_string(),
                    "studious".to_string()
                ],
                vec!["not".to_string()],
                vec![
                    "this student is studious".to_string(),
                    "the student is smart".to_string()
                ],
                vec![1, 2],
                2
            ),
            vec![1, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            top_students(
                vec![
                    "smart".to_string(),
                    "brilliant".to_string(),
                    "studious".to_string()
                ],
                vec!["not".to_string()],
                vec![
                    "this student is not studious".to_string(),
                    "the student is smart".to_string()
                ],
                vec![1, 2],
                2
            ),
            vec![2, 1]
        );
    }
}
