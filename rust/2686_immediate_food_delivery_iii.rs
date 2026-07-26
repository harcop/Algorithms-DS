/// LeetCode #2686 - Immediate Food Delivery III (MySQL)
pub const SQL: &str = r#"SELECT
    order_date,
    ROUND(
        100 * SUM(IF(customer_pref_delivery_date = order_date, 1, 0)) / COUNT(*),
        2
    ) AS immediate_percentage
FROM Delivery
GROUP BY order_date
ORDER BY order_date"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn groups_by_order_date_and_rounds() {
        let upper = SQL.to_uppercase();
        assert!(upper.contains("ORDER_DATE"));
        assert!(upper.contains("ROUND"));
        assert!(upper.contains("GROUP BY"));
        assert!(upper.contains("ORDER BY"));
        assert!(upper.contains("IMMEDIATE_PERCENTAGE"));
    }
}
