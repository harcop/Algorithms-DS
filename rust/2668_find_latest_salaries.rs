/// LeetCode #2668 - Find Latest Salaries (MySQL)
pub const SQL: &str = r#"SELECT
    emp_id,
    firstname,
    lastname,
    MAX(salary) AS salary,
    department_id
FROM Salary
GROUP BY emp_id
ORDER BY emp_id"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_max_and_group_by() {
        let upper = SQL.to_uppercase();
        assert!(upper.contains("MAX"));
        assert!(upper.contains("GROUP BY"));
        assert!(upper.contains("ORDER BY"));
    }
}
