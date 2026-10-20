package leetcode

// LeetCode #181 - Employees Earning More Than Their Managers
//
// Join each employee to their manager and keep the higher salary.
const employeesEarningMoreSQL = `SELECT e.Name AS Employee
FROM Employee e
JOIN Employee m ON e.ManagerId = m.Id
WHERE e.Salary > m.Salary`
