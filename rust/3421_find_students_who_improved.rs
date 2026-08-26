/// LeetCode #3421 - Find Students Who Improved (SQL; Rust analogue)
/// scores: (student_id, subject, score, exam_date)
fn find_students_who_improved(
    scores: Vec<(i32, String, i32, String)>,
) -> Vec<(i32, String, i32, i32)> {
    use std::collections::HashMap;
    let mut g: HashMap<(i32, String), Vec<(String, i32)>> = HashMap::new();
    for (id, subject, score, date) in scores {
        g.entry((id, subject)).or_default().push((date, score));
    }
    let mut ans = Vec::new();
    for ((id, subject), mut exams) in g {
        if exams.len() < 2 {
            continue;
        }
        exams.sort_by(|a, b| a.0.cmp(&b.0));
        let first = exams[0].1;
        let latest = exams.last().unwrap().1;
        if latest > first {
            ans.push((id, subject, first, latest));
        }
    }
    ans.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    ans
}

fn main() {
    let scores = vec![
        (101, "Math".into(), 70, "2023-01-15".into()),
        (101, "Math".into(), 85, "2023-02-15".into()),
    ];
    println!("{:?}", find_students_who_improved(scores));
}

#[cfg(test)]
mod tests {
    use super::find_students_who_improved;

    #[test]
    fn example() {
        let scores = vec![
            (101, "Math".into(), 70, "2023-01-15".into()),
            (101, "Math".into(), 85, "2023-02-15".into()),
            (101, "Physics".into(), 65, "2023-01-15".into()),
            (101, "Physics".into(), 60, "2023-02-15".into()),
            (102, "Math".into(), 80, "2023-01-15".into()),
            (102, "Math".into(), 85, "2023-02-15".into()),
            (103, "Math".into(), 90, "2023-01-15".into()),
            (104, "Physics".into(), 75, "2023-01-15".into()),
            (104, "Physics".into(), 85, "2023-02-15".into()),
        ];
        assert_eq!(
            find_students_who_improved(scores),
            vec![
                (101, "Math".into(), 70, 85),
                (102, "Math".into(), 80, 85),
                (104, "Physics".into(), 75, 85),
            ]
        );
    }
}
