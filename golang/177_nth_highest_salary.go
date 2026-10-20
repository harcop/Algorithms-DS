package leetcode

// LeetCode #177 - Nth Highest Salary
//
// Offset by N-1 after sorting the distinct salaries from high to low.
const nthHighestSalarySQL = `CREATE FUNCTION getNthHighestSalary(N INT) RETURNS INT
BEGIN
  SET N = N - 1;
  RETURN (
    SELECT DISTINCT Salary FROM Employee ORDER BY Salary DESC LIMIT 1 OFFSET N
  );
END`
