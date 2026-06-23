/// LeetCode #2066 - Account Balance (MySQL)
pub const SQL: &str = r#"SELECT
    account_id,
    day,
    SUM(IF(type = 'Deposit', amount, -amount)) OVER (
        PARTITION BY account_id
        ORDER BY day
    ) AS balance
FROM Transactions
ORDER BY 1, 2"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_window_function() {
        assert!(SQL.to_uppercase().contains("OVER"));
    }
}
