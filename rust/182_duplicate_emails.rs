/// LeetCode #182 - Duplicate Emails (MySQL)
pub const SQL: &str = r#"SELECT Email
FROM Person
GROUP BY Email
HAVING COUNT(*) > 1"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_group_by() {
        assert!(SQL.to_lowercase().contains("group by"));
    }
}
