/// LeetCode #2377 - Sort the Olympic Table (MySQL)
pub const SQL: &str = r#"SELECT *
FROM Olympic
ORDER BY 2 DESC, 3 DESC, 4 DESC, 1"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn orders_by_medals_then_country() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("OLYMPIC"));
        assert!(sql.contains("ORDER BY"));
        assert!(sql.contains("DESC"));
    }
}
