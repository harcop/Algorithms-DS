/// LeetCode #1350 - Students With Invalid Departments (SQL; Rust analogue)
use std::collections::HashSet;

fn invalid_departments(
    departments: Vec<(i32, String)>,
    students: Vec<(i32, String, i32)>,
) -> Vec<(i32, String)> {
    let valid: HashSet<i32> = departments.into_iter().map(|(id, _)| id).collect();
    students
        .into_iter()
        .filter(|(_, _, dep)| !valid.contains(dep))
        .map(|(id, name, _)| (id, name))
        .collect()
}

fn main() {
    println!("{:?}", invalid_departments(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::invalid_departments;

    #[test]
    fn example() {
        let departments = vec![
            (1, "Electrical Engineering".into()),
            (7, "Computer Engineering".into()),
            (13, "Business Administration".into()),
        ];
        let students = vec![
            (23, "Alice".into(), 1),
            (1, "Bob".into(), 7),
            (5, "Jennifer".into(), 13),
            (2, "John".into(), 14),
            (4, "Jasmine".into(), 77),
            (3, "Steve".into(), 74),
            (6, "Luis".into(), 1),
            (8, "Jonathan".into(), 7),
            (7, "Daiana".into(), 33),
            (11, "Madelynn".into(), 1),
        ];
        let mut got = invalid_departments(departments, students);
        got.sort_by_key(|r| r.0);
        assert_eq!(
            got,
            vec![
                (2, "John".into()),
                (3, "Steve".into()),
                (4, "Jasmine".into()),
                (7, "Daiana".into()),
            ]
        );
    }
}
