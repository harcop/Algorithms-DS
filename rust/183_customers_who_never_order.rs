/// LeetCode #183 - Customers Who Never Order (MySQL)
pub const SQL: &str = r#"SELECT Name AS Customers
FROM Customers c
LEFT JOIN Orders o ON c.Id = o.CustomerId
WHERE o.Id IS NULL"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn left_join() {
        assert!(SQL.to_lowercase().contains("left join"));
    }
}
