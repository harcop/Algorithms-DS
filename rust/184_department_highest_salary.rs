/// LeetCode #184 - Department Highest Salary (MySQL)
pub const SQL: &str = r#"SELECT d.Name AS Department, e.Name AS Employee, e.Salary
FROM Employee e
JOIN Department d ON e.DepartmentId = d.Id
WHERE (e.DepartmentId, e.Salary) IN (
  SELECT DepartmentId, MAX(Salary) FROM Employee GROUP BY DepartmentId
)"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn has_max_salary() {
        assert!(SQL.to_lowercase().contains("max(salary)"));
    }
}
