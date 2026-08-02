/// LeetCode #2884 - Modify Columns (Pandas; Rust analogue)
fn modify_salary(employees: Vec<(String, i32)>) -> Vec<(String, i32)> {
    employees
        .into_iter()
        .map(|(name, salary)| (name, salary * 2))
        .collect()
}

fn main() {
    let employees = vec![
        ("Jack".into(), 19666),
        ("Piper".into(), 74754),
    ];
    println!("{:?}", modify_salary(employees));
}

#[cfg(test)]
mod tests {
    use super::modify_salary;

    #[test]
    fn example() {
        let employees = vec![
            ("Jack".into(), 19666),
            ("Piper".into(), 74754),
            ("Mia".into(), 62509),
            ("Ulysses".into(), 54866),
        ];
        assert_eq!(
            modify_salary(employees),
            vec![
                ("Jack".into(), 39332),
                ("Piper".into(), 149508),
                ("Mia".into(), 125018),
                ("Ulysses".into(), 109732),
            ]
        );
    }
}
