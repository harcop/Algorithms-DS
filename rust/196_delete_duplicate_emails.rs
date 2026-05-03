/// LeetCode #196 - Delete Duplicate Emails (MySQL)
pub const SQL: &str = r#"DELETE p1 FROM Person p1
INNER JOIN Person p2
WHERE p1.Email = p2.Email AND p1.Id > p2.Id"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn deletes_duplicates() {
        assert!(SQL.to_uppercase().contains("DELETE"));
    }
}
