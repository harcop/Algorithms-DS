/// LeetCode #2084 - Drop Type 1 Orders for Customers With Type 0 Orders (MySQL)
pub const SQL: &str = r#"SELECT order_id, customer_id, order_type
FROM Orders
WHERE order_type = 0
   OR customer_id NOT IN (
        SELECT customer_id
        FROM Orders
        WHERE order_type = 0
   )
ORDER BY order_id"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn filters_type_one_orders() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("NOT IN"));
        assert!(sql.contains("ORDER_TYPE = 0"));
    }
}
