package leetcode

// LeetCode #579 - Find Cumulative Salary of an Employee
//
// Drop each employee's latest month, then sum that month with the two months before it.
const cumulativeSalarySQL = `SELECT e1.id, e1.month, SUM(e2.salary) AS Salary
FROM Employee e1
JOIN Employee e2
  ON e1.id = e2.id AND e2.month BETWEEN e1.month - 2 AND e1.month
WHERE e1.month < (SELECT MAX(month) FROM Employee WHERE id = e1.id)
GROUP BY e1.id, e1.month
ORDER BY e1.id, e1.month DESC`
