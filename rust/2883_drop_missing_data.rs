/// LeetCode #2883 - Drop Missing Data (Pandas; Rust analogue)
fn drop_missing_data(students: Vec<(i32, Option<String>, i32)>) -> Vec<(i32, String, i32)> {
    students
        .into_iter()
        .filter_map(|(id, name, age)| name.map(|name| (id, name, age)))
        .collect()
}

fn main() {
    let students = vec![
        (32, Some("Piper".into()), 5),
        (217, None, 19),
        (779, Some("Georgia".into()), 20),
    ];
    println!("{:?}", drop_missing_data(students));
}

#[cfg(test)]
mod tests {
    use super::drop_missing_data;

    #[test]
    fn example() {
        let students = vec![
            (32, Some("Piper".into()), 5),
            (217, None, 19),
            (779, Some("Georgia".into()), 20),
            (849, Some("Willow".into()), 14),
        ];
        assert_eq!(
            drop_missing_data(students),
            vec![
                (32, "Piper".into(), 5),
                (779, "Georgia".into(), 20),
                (849, "Willow".into(), 14),
            ]
        );
    }
}
