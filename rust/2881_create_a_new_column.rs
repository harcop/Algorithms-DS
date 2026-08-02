/// LeetCode #2881 - Create a New Column (Pandas; Rust analogue)
fn create_bonus(employees: Vec<(String, i32)>) -> Vec<(String, i32, i32)> {
    employees
        .into_iter()
        .map(|(name, salary)| (name, salary, salary * 2))
        .collect()
}

fn main() {
    let employees = vec![
        ("Piper".into(), 4548),
        ("Grace".into(), 28150),
    ];
    println!("{:?}", create_bonus(employees));
}

#[cfg(test)]
mod tests {
    use super::create_bonus;

    #[test]
    fn example() {
        let employees = vec![
            ("Piper".into(), 4548),
            ("Grace".into(), 28150),
            ("Georgia".into(), 1103),
            ("Willow".into(), 6593),
            ("Finn".into(), 74576),
            ("Thomas".into(), 24433),
        ];
        assert_eq!(
            create_bonus(employees),
            vec![
                ("Piper".into(), 4548, 9096),
                ("Grace".into(), 28150, 56300),
                ("Georgia".into(), 1103, 2206),
                ("Willow".into(), 6593, 13186),
                ("Finn".into(), 74576, 149152),
                ("Thomas".into(), 24433, 48866),
            ]
        );
    }
}
