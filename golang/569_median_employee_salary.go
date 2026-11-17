package leetcode

// LeetCode #569 - Median Employee Salary
//
// Rank salaries inside each company and keep the one or two rows that sit in the middle.
const medianEmployeeSalarySQL = `SELECT id, company, salary
FROM (
  SELECT id, company, salary,
         ROW_NUMBER() OVER (PARTITION BY company ORDER BY salary, id) AS rn,
         COUNT(*) OVER (PARTITION BY company) AS cnt
  FROM Employee
) ranked
WHERE rn IN (FLOOR((cnt + 1) / 2), FLOOR((cnt + 2) / 2))`
