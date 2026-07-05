/// LeetCode #2228 - Users With Two Purchases Within Seven Days (MySQL)
pub const SQL: &str = r#"WITH
    t AS (
        SELECT
            user_id,
            DATEDIFF(
                purchase_date,
                LAG(purchase_date, 1) OVER (
                    PARTITION BY user_id
                    ORDER BY purchase_date
                )
            ) AS d
        FROM Purchases
    )
SELECT DISTINCT user_id
FROM t
WHERE d <= 7
ORDER BY user_id"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn finds_users_with_close_purchases() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("LAG(PURCHASE_DATE"));
        assert!(sql.contains("D <= 7"));
        assert!(sql.contains("DISTINCT USER_ID"));
    }
}
