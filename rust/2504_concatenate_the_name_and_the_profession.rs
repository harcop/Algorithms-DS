/// LeetCode #2504 - Concatenate the Name and the Profession (MySQL)
pub const SQL: &str = r#"SELECT person_id, CONCAT(name, "(", SUBSTRING(profession, 1, 1), ")") AS name
FROM Person
ORDER BY person_id DESC"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn concatenates_name_and_profession() {
        assert!(SQL.contains("CONCAT"));
        assert!(SQL.contains("Person"));
        assert!(SQL.contains("DESC"));
    }
}
