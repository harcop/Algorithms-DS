/// LeetCode #580 - Count Student Number in Departments (SQL; Rust analogue)
use std::collections::HashMap;

fn count_students(
    student: Vec<(i32, String, String, i32)>,
    department: Vec<(i32, String)>,
) -> Vec<(String, i32)> {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for (_, _, _, dept_id) in student {
        *cnt.entry(dept_id).or_insert(0) += 1;
    }
    let mut ans: Vec<(String, i32)> = department
        .into_iter()
        .map(|(id, name)| (name, *cnt.get(&id).unwrap_or(&0)))
        .collect();
    ans.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::count_students;

    #[test]
    fn example() {
        let student = vec![
            (1, "Jack".into(), "M".into(), 1),
            (2, "Jane".into(), "F".into(), 1),
            (3, "Mark".into(), "M".into(), 2),
        ];
        let department = vec![
            (1, "Engineering".into()),
            (2, "Science".into()),
            (3, "Law".into()),
        ];
        assert_eq!(
            count_students(student, department),
            vec![
                ("Engineering".into(), 2),
                ("Science".into(), 1),
                ("Law".into(), 0),
            ]
        );
    }
}
