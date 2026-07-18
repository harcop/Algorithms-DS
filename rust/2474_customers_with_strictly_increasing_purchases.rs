/// LeetCode #2474 - Customers With Strictly Increasing Purchases (MySQL)
pub const SQL: &str = r#"SELECT
    customer_id
FROM
    (
        SELECT
            customer_id,
            YEAR(order_date),
            SUM(price) AS total,
            YEAR(order_date) - RANK() OVER (
                PARTITION BY customer_id
                ORDER BY SUM(price)
            ) AS rk
        FROM Orders
        GROUP BY customer_id, YEAR(order_date)
    ) AS t
GROUP BY customer_id
HAVING COUNT(DISTINCT rk) = 1"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_rank_window() {
        assert!(SQL.to_uppercase().contains("RANK()"));
    }
}
