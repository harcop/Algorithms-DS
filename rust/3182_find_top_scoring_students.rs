/// LeetCode #3182 - Find Top Scoring Students (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

/// students: (student_id, name, major)
/// courses: (course_id, name, credits, major)
/// enrollments: (student_id, course_id, semester, grade)
fn find_top_scoring_students(
    students: Vec<(i32, String, String)>,
    courses: Vec<(i32, String, i32, String)>,
    enrollments: Vec<(i32, i32, String, String)>,
) -> Vec<i32> {
    let mut major_courses: HashMap<String, HashSet<i32>> = HashMap::new();
    for (cid, _, _, major) in courses {
        major_courses.entry(major).or_default().insert(cid);
    }
    let mut got_a: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (sid, cid, _, grade) in enrollments {
        if grade == "A" {
            got_a.entry(sid).or_default().insert(cid);
        }
    }
    let mut ans: Vec<i32> = students
        .into_iter()
        .filter_map(|(sid, _, major)| {
            let need = major_courses.get(&major)?;
            let have = got_a.get(&sid).cloned().unwrap_or_default();
            if need.iter().all(|c| have.contains(c)) {
                Some(sid)
            } else {
                None
            }
        })
        .collect();
    ans.sort_unstable();
    ans
}

fn main() {
    let students = vec![
        (1, "Alice".into(), "CS".into()),
        (2, "Bob".into(), "CS".into()),
        (3, "Charlie".into(), "Math".into()),
        (4, "David".into(), "Math".into()),
    ];
    let courses = vec![
        (101, "Algo".into(), 3, "CS".into()),
        (102, "DS".into(), 3, "CS".into()),
        (103, "Calc".into(), 4, "Math".into()),
        (104, "LA".into(), 4, "Math".into()),
    ];
    let enrollments = vec![
        (1, 101, "F23".into(), "A".into()),
        (1, 102, "F23".into(), "A".into()),
        (2, 101, "F23".into(), "B".into()),
        (2, 102, "F23".into(), "A".into()),
        (3, 103, "F23".into(), "A".into()),
        (3, 104, "F23".into(), "A".into()),
        (4, 103, "F23".into(), "A".into()),
        (4, 104, "F23".into(), "B".into()),
    ];
    println!("{:?}", find_top_scoring_students(students, courses, enrollments));
}

#[cfg(test)]
mod tests {
    use super::find_top_scoring_students;

    #[test]
    fn example() {
        let students = vec![
            (1, "Alice".into(), "CS".into()),
            (2, "Bob".into(), "CS".into()),
            (3, "Charlie".into(), "Math".into()),
            (4, "David".into(), "Math".into()),
        ];
        let courses = vec![
            (101, "Algo".into(), 3, "CS".into()),
            (102, "DS".into(), 3, "CS".into()),
            (103, "Calc".into(), 4, "Math".into()),
            (104, "LA".into(), 4, "Math".into()),
        ];
        let enrollments = vec![
            (1, 101, "F23".into(), "A".into()),
            (1, 102, "F23".into(), "A".into()),
            (2, 101, "F23".into(), "B".into()),
            (2, 102, "F23".into(), "A".into()),
            (3, 103, "F23".into(), "A".into()),
            (3, 104, "F23".into(), "A".into()),
            (4, 103, "F23".into(), "A".into()),
            (4, 104, "F23".into(), "B".into()),
        ];
        assert_eq!(
            find_top_scoring_students(students, courses, enrollments),
            vec![1, 3]
        );
    }
}
