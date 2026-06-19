/// LeetCode #1978 - Employees Whose Manager Left the Company (MySQL)
pub const SQL: &str = r#"SELECT e1.employee_id
FROM Employees AS e1
LEFT JOIN Employees AS e2 ON e1.manager_id = e2.employee_id
WHERE e1.salary < 30000 AND e1.manager_id IS NOT NULL AND e2.employee_id IS NULL
ORDER BY 1"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn filters_by_salary() {
        assert!(SQL.contains("30000"));
    }
}
