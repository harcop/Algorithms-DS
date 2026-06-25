/// LeetCode #2082 - The Number of Rich Customers (MySQL)
pub const SQL: &str = r#"SELECT COUNT(DISTINCT customer_id) AS rich_count
FROM Store
WHERE amount > 500"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn counts_distinct_customers() {
        assert!(SQL.to_uppercase().contains("COUNT(DISTINCT CUSTOMER_ID)"));
    }
}
