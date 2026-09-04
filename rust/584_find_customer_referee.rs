/// LeetCode #584 - Find Customer Referee (SQL; Rust analogue)
fn find_customer_referee(customer: Vec<(i32, String, Option<i32>)>) -> Vec<String> {
    customer
        .into_iter()
        .filter(|(_, _, r)| r != &Some(2))
        .map(|(_, name, _)| name)
        .collect()
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::find_customer_referee;

    #[test]
    fn example() {
        let customer = vec![
            (1, "Will".into(), None),
            (2, "Jane".into(), None),
            (3, "Alex".into(), Some(2)),
            (4, "Bill".into(), None),
            (5, "Zack".into(), Some(1)),
            (6, "Mark".into(), Some(2)),
        ];
        assert_eq!(
            find_customer_referee(customer),
            vec![
                "Will".to_string(),
                "Jane".to_string(),
                "Bill".to_string(),
                "Zack".to_string(),
            ]
        );
    }
}
