/// LeetCode #185 - Department Top Three Salaries (MySQL)
pub const SQL: &str = r#"SELECT d.Name AS Department, e.Name AS Employee, e.Salary
FROM Employee e
JOIN Department d ON e.DepartmentId = d.Id
WHERE (
  SELECT COUNT(DISTINCT e2.Salary)
  FROM Employee e2
  WHERE e2.DepartmentId = e.DepartmentId AND e2.Salary >= e.Salary
) <= 3"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn top_three_pattern() {
        assert!(SQL.contains("<= 3"));
    }
}
