/// LeetCode #2886 - Change Data Type (Pandas; Rust analogue)
fn change_data_type(
    students: Vec<(i32, String, i32, f64)>,
) -> Vec<(i32, String, i32, i32)> {
    students
        .into_iter()
        .map(|(id, name, age, grade)| (id, name, age, grade as i32))
        .collect()
}

fn main() {
    let students = vec![
        (1, "Ava".into(), 6, 73.0),
        (2, "Kate".into(), 15, 87.0),
    ];
    println!("{:?}", change_data_type(students));
}

#[cfg(test)]
mod tests {
    use super::change_data_type;

    #[test]
    fn example() {
        let students = vec![
            (1, "Ava".into(), 6, 73.0),
            (2, "Kate".into(), 15, 87.0),
        ];
        assert_eq!(
            change_data_type(students),
            vec![(1, "Ava".into(), 6, 73), (2, "Kate".into(), 15, 87)]
        );
    }
}
