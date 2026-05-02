/// LeetCode #175 - Combine Two Tables (MySQL)
pub const SQL: &str = r#"SELECT FirstName, LastName, City, State
FROM Person
LEFT JOIN Address ON Person.PersonId = Address.PersonId"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn has_join() {
        assert!(SQL.to_lowercase().contains("left join"));
    }
}
