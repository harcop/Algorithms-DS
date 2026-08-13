/// LeetCode #3188 - Find Top Scoring Students II (SQL; Rust analogue)
use std::collections::HashMap;

/// students: (student_id, name, major)
/// courses: (course_id, name, credits, major, mandatory)
/// enrollments: (student_id, course_id, semester, grade, gpa)
fn find_top_scoring_students_ii(
    students: Vec<(i32, String, String)>,
    courses: Vec<(i32, String, i32, String, bool)>,
    enrollments: Vec<(i32, i32, String, String, f64)>,
) -> Vec<i32> {
    let mut gpa_sum: HashMap<i32, (f64, i32)> = HashMap::new();
    let mut grade: HashMap<(i32, i32), String> = HashMap::new();
    for (sid, cid, _, g, gpa) in enrollments {
        grade.insert((sid, cid), g);
        let e = gpa_sum.entry(sid).or_insert((0.0, 0));
        e.0 += gpa;
        e.1 += 1;
    }
    let mut mandatory: HashMap<String, Vec<i32>> = HashMap::new();
    let mut elective: HashMap<String, Vec<i32>> = HashMap::new();
    for (cid, _, _, major, mand) in courses {
        if mand {
            mandatory.entry(major).or_default().push(cid);
        } else {
            elective.entry(major).or_default().push(cid);
        }
    }
    let mut ans: Vec<i32> = students
        .into_iter()
        .filter_map(|(sid, _, major)| {
            let (sum, cnt) = *gpa_sum.get(&sid)?;
            if cnt == 0 || sum / (cnt as f64) < 2.5 {
                return None;
            }
            let mand = mandatory.get(&major).cloned().unwrap_or_default();
            if !mand.iter().all(|&c| grade.get(&(sid, c)).map(|g| g.as_str()) == Some("A")) {
                return None;
            }
            let elec = elective.get(&major).cloned().unwrap_or_default();
            let taken: Vec<&str> = elec
                .iter()
                .filter_map(|&c| grade.get(&(sid, c)).map(|g| g.as_str()))
                .collect();
            if taken.len() < 2 {
                return None;
            }
            if taken.iter().all(|&g| g == "A" || g == "B") {
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
    println!("{:?}", find_top_scoring_students_ii(vec![], vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_top_scoring_students_ii;

    #[test]
    fn example() {
        let students = vec![
            (1, "Alice".into(), "CS".into()),
            (2, "Bob".into(), "CS".into()),
            (3, "Charlie".into(), "Math".into()),
            (4, "David".into(), "Math".into()),
        ];
        let courses = vec![
            (101, "Algo".into(), 3, "CS".into(), true),
            (102, "DS".into(), 3, "CS".into(), true),
            (103, "Calc".into(), 4, "Math".into(), true),
            (104, "LA".into(), 4, "Math".into(), true),
            (105, "ML".into(), 3, "CS".into(), false),
            (106, "Prob".into(), 3, "Math".into(), false),
            (107, "OS".into(), 3, "CS".into(), false),
            (108, "Stat".into(), 3, "Math".into(), false),
        ];
        let enrollments = vec![
            (1, 101, "F23".into(), "A".into(), 4.0),
            (1, 102, "S23".into(), "A".into(), 4.0),
            (1, 105, "S23".into(), "A".into(), 4.0),
            (1, 107, "F23".into(), "B".into(), 3.5),
            (2, 101, "F23".into(), "A".into(), 4.0),
            (2, 102, "S23".into(), "B".into(), 3.0),
            (3, 103, "F23".into(), "A".into(), 4.0),
            (3, 104, "S23".into(), "A".into(), 4.0),
            (3, 106, "S23".into(), "A".into(), 4.0),
            (3, 108, "F23".into(), "B".into(), 3.5),
            (4, 103, "F23".into(), "B".into(), 3.0),
            (4, 104, "S23".into(), "B".into(), 3.0),
        ];
        assert_eq!(
            find_top_scoring_students_ii(students, courses, enrollments),
            vec![1, 3]
        );
    }
}
