package leetcode

// LeetCode #185 - Department Top Three Salaries
//
// Keep a salary when fewer than three distinct higher-or-equal salaries exist in that department.
const departmentTopThreeSalariesSQL = `SELECT d.Name AS Department, e.Name AS Employee, e.Salary
FROM Employee e
JOIN Department d ON e.DepartmentId = d.Id
WHERE (
  SELECT COUNT(DISTINCT e2.Salary)
  FROM Employee e2
  WHERE e2.DepartmentId = e.DepartmentId AND e2.Salary >= e.Salary
) <= 3`
