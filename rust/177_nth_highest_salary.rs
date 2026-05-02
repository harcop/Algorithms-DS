/// LeetCode #177 - Nth Highest Salary (MySQL function)
pub const SQL: &str = r#"CREATE FUNCTION getNthHighestSalary(N INT) RETURNS INT
BEGIN
  SET N = N - 1;
  RETURN (
    SELECT DISTINCT Salary FROM Employee ORDER BY Salary DESC LIMIT 1 OFFSET N
  );
END"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn defines_function() {
        assert!(SQL.to_lowercase().contains("function"));
    }
}
