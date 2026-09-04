/// LeetCode #1412 - Find the Quiet Students in All Exams (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn quiet_students(
    students: Vec<(i32, String)>,
    exams: Vec<(i32, i32, i32)>,
) -> Vec<(i32, String)> {
    let mut by_exam: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    for (eid, sid, score) in exams {
        by_exam.entry(eid).or_default().push((sid, score));
    }
    let mut took: HashSet<i32> = HashSet::new();
    let mut extreme: HashSet<i32> = HashSet::new();
    for rows in by_exam.values() {
        let min = rows.iter().map(|r| r.1).min().unwrap();
        let max = rows.iter().map(|r| r.1).max().unwrap();
        for &(sid, score) in rows {
            took.insert(sid);
            if score == min || score == max {
                extreme.insert(sid);
            }
        }
    }
    let names: HashMap<i32, String> = students.into_iter().collect();
    let mut ans: Vec<(i32, String)> = took
        .into_iter()
        .filter(|id| !extreme.contains(id))
        .map(|id| (id, names[&id].clone()))
        .collect();
    ans.sort_by_key(|r| r.0);
    ans
}

fn main() {
    println!("{:?}", quiet_students(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::quiet_students;

    #[test]
    fn example() {
        let students = vec![
            (1, "Daniel".into()),
            (2, "Jade".into()),
            (3, "Stella".into()),
            (4, "Jonathan".into()),
            (5, "Will".into()),
        ];
        let exams = vec![
            (10, 1, 70),
            (10, 2, 80),
            (10, 3, 90),
            (20, 1, 80),
            (30, 1, 70),
            (30, 3, 80),
            (30, 4, 90),
            (40, 1, 60),
            (40, 2, 70),
            (40, 4, 80),
        ];
        assert_eq!(quiet_students(students, exams), vec![(2, "Jade".into())]);
    }
}
