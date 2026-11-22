package leetcode

// LeetCode #627 - Swap Salary
//
// Every m becomes f, and every f becomes m.
const swapSalarySQL = `UPDATE Salary
SET sex = CASE sex
    WHEN 'm' THEN 'f'
    ELSE 'm'
END`
