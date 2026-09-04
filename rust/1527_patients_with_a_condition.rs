/// LeetCode #1527 - Patients With a Condition (SQL; Rust analogue)
fn patients_with_condition(patients: Vec<(i32, String, String)>) -> Vec<(i32, String, String)> {
    patients
        .into_iter()
        .filter(|(_, _, cond)| {
            cond.split_whitespace().any(|w| w.starts_with("DIAB1"))
        })
        .collect()
}

fn main() {
    println!("{:?}", patients_with_condition(vec![]));
}

#[cfg(test)]
mod tests {
    use super::patients_with_condition;

    #[test]
    fn example() {
        let patients = vec![
            (1, "Daniel".into(), "YFEV COUGH".into()),
            (2, "Alice".into(), "".into()),
            (3, "Bob".into(), "DIAB100 MYOP".into()),
            (4, "George".into(), "ACNE DIAB100".into()),
            (5, "Alain".into(), "DIAB201".into()),
        ];
        assert_eq!(
            patients_with_condition(patients),
            vec![
                (3, "Bob".into(), "DIAB100 MYOP".into()),
                (4, "George".into(), "ACNE DIAB100".into()),
            ]
        );
    }
}
