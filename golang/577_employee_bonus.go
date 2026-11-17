package leetcode

// LeetCode #577 - Employee Bonus
//
// Keep employees whose bonus is missing or below 1000.
const employeeBonusSQL = `SELECT e.name, b.bonus
FROM Employee e
LEFT JOIN Bonus b ON e.empId = b.empId
WHERE b.bonus < 1000 OR b.bonus IS NULL`
