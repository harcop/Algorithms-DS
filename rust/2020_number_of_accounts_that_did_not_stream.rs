/// LeetCode #2020 - Number of Accounts That Did Not Stream (MySQL)
pub const SQL: &str = r#"SELECT COUNT(sub.account_id) AS accounts_count
FROM Subscriptions AS sub
LEFT JOIN Streams USING (account_id)
WHERE
    YEAR(start_date) <= 2021
    AND YEAR(end_date) >= 2021
    AND (YEAR(stream_date) != 2021 OR stream_date > end_date)"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_left_join() {
        assert!(SQL.to_uppercase().contains("LEFT JOIN"));
    }
}
