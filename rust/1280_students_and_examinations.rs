/// LeetCode #1280 - Students and Examinations (SQL; Rust analogue)
use std::collections::HashMap;

fn students_and_examinations(
    students: Vec<(i32, String)>,
    subjects: Vec<String>,
    examinations: Vec<(i32, String)>,
) -> Vec<(i32, String, String, i32)> {
    let mut cnt: HashMap<(i32, String), i32> = HashMap::new();
    for (sid, sub) in examinations {
        *cnt.entry((sid, sub)).or_insert(0) += 1;
    }
    let mut ans = Vec::new();
    for (sid, sname) in &students {
        for sub in &subjects {
            ans.push((
                *sid,
                sname.clone(),
                sub.clone(),
                *cnt.get(&(*sid, sub.clone())).unwrap_or(&0),
            ));
        }
    }
    ans.sort_by(|a, b| a.0.cmp(&b.0).then(a.2.cmp(&b.2)));
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::students_and_examinations;

    #[test]
    fn example() {
        let students = vec![
            (1, "Alice".into()),
            (2, "Bob".into()),
            (13, "John".into()),
            (6, "Alex".into()),
        ];
        let subjects = vec!["Math".into(), "Physics".into(), "Programming".into()];
        let examinations = vec![
            (1, "Math".into()),
            (1, "Physics".into()),
            (1, "Programming".into()),
            (2, "Programming".into()),
            (1, "Physics".into()),
            (1, "Math".into()),
            (13, "Math".into()),
            (13, "Programming".into()),
            (13, "Physics".into()),
            (2, "Math".into()),
            (1, "Math".into()),
        ];
        assert_eq!(
            students_and_examinations(students, subjects, examinations),
            vec![
                (1, "Alice".into(), "Math".into(), 3),
                (1, "Alice".into(), "Physics".into(), 2),
                (1, "Alice".into(), "Programming".into(), 1),
                (2, "Bob".into(), "Math".into(), 1),
                (2, "Bob".into(), "Physics".into(), 0),
                (2, "Bob".into(), "Programming".into(), 1),
                (6, "Alex".into(), "Math".into(), 0),
                (6, "Alex".into(), "Physics".into(), 0),
                (6, "Alex".into(), "Programming".into(), 0),
                (13, "John".into(), "Math".into(), 1),
                (13, "John".into(), "Physics".into(), 1),
                (13, "John".into(), "Programming".into(), 1),
            ]
        );
    }
}
