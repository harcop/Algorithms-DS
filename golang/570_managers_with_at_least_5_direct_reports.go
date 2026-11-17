package leetcode

// LeetCode #570 - Managers with at Least 5 Direct Reports
//
// A manager is anyone whose id is listed as managerId at least five times.
const managersWithAtLeast5ReportsSQL = `SELECT name
FROM Employee
WHERE id IN (
  SELECT managerId
  FROM Employee
  GROUP BY managerId
  HAVING COUNT(*) >= 5
)`
