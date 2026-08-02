/// LeetCode #2880 - Select Data (Pandas; Rust analogue)
fn select_data(students: Vec<(i32, String, i32)>) -> Vec<(String, i32)> {
    students
        .into_iter()
        .filter(|(student_id, _, _)| *student_id == 101)
        .map(|(_, name, age)| (name, age))
        .collect()
}

fn main() {
    let students = vec![
        (101, "Ulysses".into(), 13),
        (53, "William".into(), 10),
        (128, "Henry".into(), 6),
        (3, "Henry".into(), 11),
    ];
    println!("{:?}", select_data(students));
}

#[cfg(test)]
mod tests {
    use super::select_data;

    #[test]
    fn example() {
        let students = vec![
            (101, "Ulysses".into(), 13),
            (53, "William".into(), 10),
            (128, "Henry".into(), 6),
            (3, "Henry".into(), 11),
        ];
        assert_eq!(select_data(students), vec![("Ulysses".into(), 13)]);
    }
}
