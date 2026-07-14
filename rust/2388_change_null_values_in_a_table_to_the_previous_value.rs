/// LeetCode #2388 - Change Null Values in a Table to the Previous Value (MySQL)
pub const SQL: &str = r#"SELECT
    id,
    CASE
        WHEN drink IS NOT NULL THEN @cur := drink
        ELSE @cur
    END AS drink
FROM CoffeeShop"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn fills_null_drink_from_previous() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("COFFEESHOP"));
        assert!(sql.contains("DRINK"));
        assert!(sql.contains("CASE"));
        assert!(sql.contains("@CUR"));
    }
}
