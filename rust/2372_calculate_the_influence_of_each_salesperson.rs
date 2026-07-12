/// LeetCode #2372 - Calculate the Influence of Each Salesperson (MySQL)
pub const SQL: &str = r#"SELECT
    Salesperson.salesperson_id,
    Salesperson.name,
    SUM(IFNULL(Sales.price, 0)) AS total
FROM Salesperson
LEFT JOIN Customer
    USING (salesperson_id)
LEFT JOIN Sales
    USING (customer_id)
GROUP BY 1"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn joins_salesperson_customer_sales() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("SALESPERSON"));
        assert!(sql.contains("CUSTOMER"));
        assert!(sql.contains("SALES"));
        assert!(sql.contains("LEFT JOIN"));
        assert!(sql.contains("SUM"));
        assert!(sql.contains("GROUP BY"));
    }
}
