/// LeetCode #2885 - Rename Columns (Pandas; Rust analogue)
fn rename_columns(
    students: Vec<(i32, String, String, i32)>,
) -> Vec<(i32, String, String, i32)> {
    // Columns conceptually renamed:
    // id -> student_id, first -> first_name, last -> last_name, age -> age_in_years
    students
}

fn main() {
    let students = vec![
        (1, "Mason".into(), "King".into(), 6),
        (2, "Ava".into(), "Wright".into(), 7),
    ];
    println!("{:?}", rename_columns(students));
}

#[cfg(test)]
mod tests {
    use super::rename_columns;

    #[test]
    fn example() {
        let students = vec![
            (1, "Mason".into(), "King".into(), 6),
            (2, "Ava".into(), "Wright".into(), 7),
            (3, "Taylor".into(), "Hall".into(), 16),
            (4, "Georgia".into(), "Thompson".into(), 18),
            (5, "Thomas".into(), "Moore".into(), 10),
        ];
        assert_eq!(
            rename_columns(students),
            vec![
                (1, "Mason".into(), "King".into(), 6),
                (2, "Ava".into(), "Wright".into(), 7),
                (3, "Taylor".into(), "Hall".into(), 16),
                (4, "Georgia".into(), "Thompson".into(), 18),
                (5, "Thomas".into(), "Moore".into(), 10),
            ]
        );
    }
}
