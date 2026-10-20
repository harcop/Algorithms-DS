package leetcode

// LeetCode #176 - Second Highest Salary
//
// The second highest salary is the maximum salary strictly below the maximum.
const secondHighestSalarySQL = `SELECT MAX(Salary) AS SecondHighestSalary
FROM Employee
WHERE Salary < (SELECT MAX(Salary) FROM Employee)`
