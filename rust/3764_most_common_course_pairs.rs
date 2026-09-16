/// LeetCode #3764 - Most Common Course Pairs (SQL; Rust analogue)
use std::collections::HashMap;

/// row: (user_id, course_id, course_name, completion_date, course_rating)
fn most_common_course_pairs(
    completions: Vec<(i32, i32, String, String, i32)>,
) -> Vec<(String, String, i32)> {
    let mut by_user: HashMap<i32, Vec<(String, String, i32)>> = HashMap::new();
    for (uid, _cid, name, date, rating) in completions {
        by_user.entry(uid).or_default().push((name, date, rating));
    }
    let mut pair_cnt: HashMap<(String, String), i32> = HashMap::new();
    for rows in by_user.values_mut() {
        if rows.len() < 5 {
            continue;
        }
        let avg = rows.iter().map(|r| r.2 as f64).sum::<f64>() / rows.len() as f64;
        if avg < 4.0 {
            continue;
        }
        rows.sort_by(|a, b| a.1.cmp(&b.1));
        for w in rows.windows(2) {
            *pair_cnt
                .entry((w[0].0.clone(), w[1].0.clone()))
                .or_insert(0) += 1;
        }
    }
    let mut ans: Vec<(String, String, i32)> = pair_cnt
        .into_iter()
        .map(|((a, b), c)| (a, b, c))
        .collect();
    ans.sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(&b.0)).then(a.1.cmp(&b.1)));
    ans
}

fn main() {
    println!("{:?}", most_common_course_pairs(vec![]));
}

#[cfg(test)]
mod tests {
    use super::most_common_course_pairs;

    #[test]
    fn example() {
        let rows = vec![
            (1, 101, "Python Basics".into(), "2024-01-05".into(), 5),
            (1, 102, "SQL Fundamentals".into(), "2024-02-10".into(), 4),
            (1, 103, "JavaScript".into(), "2024-03-15".into(), 5),
            (1, 104, "React Basics".into(), "2024-04-20".into(), 4),
            (1, 105, "Node.js".into(), "2024-05-25".into(), 5),
            (1, 106, "Docker".into(), "2024-06-30".into(), 4),
            (2, 101, "Python Basics".into(), "2024-01-08".into(), 4),
            (2, 104, "React Basics".into(), "2024-02-14".into(), 5),
            (2, 105, "Node.js".into(), "2024-03-20".into(), 4),
            (2, 106, "Docker".into(), "2024-04-25".into(), 5),
            (2, 107, "AWS Fundamentals".into(), "2024-05-30".into(), 4),
            (3, 101, "Python Basics".into(), "2024-01-10".into(), 3),
            (3, 102, "SQL Fundamentals".into(), "2024-02-12".into(), 3),
            (3, 103, "JavaScript".into(), "2024-03-18".into(), 3),
            (3, 104, "React Basics".into(), "2024-04-22".into(), 2),
            (3, 105, "Node.js".into(), "2024-05-28".into(), 3),
            (4, 101, "Python Basics".into(), "2024-01-12".into(), 5),
            (4, 108, "Data Science".into(), "2024-02-16".into(), 5),
            (4, 109, "Machine Learning".into(), "2024-03-22".into(), 5),
        ];
        assert_eq!(
            most_common_course_pairs(rows),
            vec![
                ("Node.js".into(), "Docker".into(), 2),
                ("React Basics".into(), "Node.js".into(), 2),
                ("Docker".into(), "AWS Fundamentals".into(), 1),
                ("JavaScript".into(), "React Basics".into(), 1),
                ("Python Basics".into(), "React Basics".into(), 1),
                ("Python Basics".into(), "SQL Fundamentals".into(), 1),
                ("SQL Fundamentals".into(), "JavaScript".into(), 1),
            ]
        );
    }
}
