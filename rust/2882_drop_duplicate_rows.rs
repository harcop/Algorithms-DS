/// LeetCode #2882 - Drop Duplicate Rows (Pandas; Rust analogue)
fn drop_duplicate_emails(customers: Vec<(i32, String, String)>) -> Vec<(i32, String, String)> {
    use std::collections::HashSet;

    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for row in customers {
        if seen.insert(row.2.clone()) {
            result.push(row);
        }
    }
    result
}

fn main() {
    let customers = vec![
        (1, "Ella".into(), "emily@example.com".into()),
        (2, "David".into(), "michael@example.com".into()),
        (4, "Alice".into(), "john@example.com".into()),
        (5, "Finn".into(), "john@example.com".into()),
    ];
    println!("{:?}", drop_duplicate_emails(customers));
}

#[cfg(test)]
mod tests {
    use super::drop_duplicate_emails;

    #[test]
    fn example() {
        let customers = vec![
            (1, "Ella".into(), "emily@example.com".into()),
            (2, "David".into(), "michael@example.com".into()),
            (3, "Zachary".into(), "sarah@example.com".into()),
            (4, "Alice".into(), "john@example.com".into()),
            (5, "Finn".into(), "john@example.com".into()),
            (6, "Violet".into(), "alice@example.com".into()),
        ];
        assert_eq!(
            drop_duplicate_emails(customers),
            vec![
                (1, "Ella".into(), "emily@example.com".into()),
                (2, "David".into(), "michael@example.com".into()),
                (3, "Zachary".into(), "sarah@example.com".into()),
                (4, "Alice".into(), "john@example.com".into()),
                (6, "Violet".into(), "alice@example.com".into()),
            ]
        );
    }
}
