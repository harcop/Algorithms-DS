/// LeetCode #596 - Classes More Than 5 Students (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn classes_more_than_5(courses: Vec<(String, String)>) -> Vec<String> {
    let mut by_class: HashMap<String, HashSet<String>> = HashMap::new();
    for (student, class) in courses {
        by_class.entry(class).or_default().insert(student);
    }
    let mut ans: Vec<String> = by_class
        .into_iter()
        .filter(|(_, s)| s.len() >= 5)
        .map(|(c, _)| c)
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::classes_more_than_5;

    #[test]
    fn example() {
        let courses = vec![
            ("A".into(), "Math".into()),
            ("B".into(), "English".into()),
            ("C".into(), "Math".into()),
            ("D".into(), "Biology".into()),
            ("E".into(), "Math".into()),
            ("F".into(), "Computer".into()),
            ("G".into(), "Math".into()),
            ("H".into(), "Math".into()),
            ("I".into(), "Math".into()),
        ];
        assert_eq!(classes_more_than_5(courses), vec!["Math".to_string()]);
    }
}
