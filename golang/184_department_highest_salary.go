package leetcode

// LeetCode #184 - Department Highest Salary
//
// Keep employees whose salary is the maximum inside their department.
const departmentHighestSalarySQL = `SELECT d.Name AS Department, e.Name AS Employee, e.Salary
FROM Employee e
JOIN Department d ON e.DepartmentId = d.Id
WHERE (e.DepartmentId, e.Salary) IN (
  SELECT DepartmentId, MAX(Salary) FROM Employee GROUP BY DepartmentId
)`
