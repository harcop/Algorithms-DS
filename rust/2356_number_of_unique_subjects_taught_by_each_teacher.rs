/// LeetCode #2356 - Number of Unique Subjects Taught by Each Teacher (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn unique_subjects(teacher: Vec<(i32, i32, i32)>) -> Vec<(i32, i32)> {
    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (teacher_id, subject_id, _dept_id) in teacher {
        map.entry(teacher_id).or_default().insert(subject_id);
    }
    let mut ans: Vec<(i32, i32)> = map
        .into_iter()
        .map(|(tid, subs)| (tid, subs.len() as i32))
        .collect();
    ans.sort();
    ans
}

fn main() {
    let teacher = vec![
        (1, 2, 3),
        (1, 2, 4),
        (1, 3, 3),
        (2, 1, 1),
        (2, 2, 1),
        (2, 3, 1),
        (2, 4, 1),
    ];
    println!("{:?}", unique_subjects(teacher));
}

#[cfg(test)]
mod tests {
    use super::unique_subjects;

    #[test]
    fn example_one() {
        let teacher = vec![
            (1, 2, 3),
            (1, 2, 4),
            (1, 3, 3),
            (2, 1, 1),
            (2, 2, 1),
            (2, 3, 1),
            (2, 4, 1),
        ];
        assert_eq!(unique_subjects(teacher), vec![(1, 2), (2, 4)]);
    }
}
