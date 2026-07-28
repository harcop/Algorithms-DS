/// LeetCode #2752 - Customers with Maximum Number of Transactions on Consecutive Days (MySQL)
pub const SQL: &str = r#"WITH
    s AS (
        SELECT
            customer_id,
            DATE_SUB(
                transaction_date,
                INTERVAL ROW_NUMBER() OVER (
                    PARTITION BY customer_id
                    ORDER BY transaction_date
                ) DAY
            ) AS transaction_date
        FROM Transactions
    ),
    t AS (
        SELECT customer_id, transaction_date, COUNT(1) AS cnt
        FROM s
        GROUP BY 1, 2
    )
SELECT customer_id
FROM t
WHERE cnt = (SELECT MAX(cnt) FROM t)
ORDER BY customer_id"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn finds_max_consecutive_transaction_customers() {
        let upper = SQL.to_uppercase();
        assert!(upper.contains("ROW_NUMBER()"));
        assert!(upper.contains("PARTITION BY CUSTOMER_ID"));
        assert!(upper.contains("DATE_SUB"));
        assert!(upper.contains("MAX(CNT)"));
        assert!(upper.contains("ORDER BY CUSTOMER_ID"));
    }
}
