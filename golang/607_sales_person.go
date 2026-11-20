package leetcode

// LeetCode #607 - Sales Person
//
// Drop anyone whose orders include the company named RED.
const salesPersonSQL = `SELECT name
FROM SalesPerson
WHERE sales_id NOT IN (
    SELECT o.sales_id
    FROM Orders o
    JOIN Company c ON o.com_id = c.com_id
    WHERE c.name = 'RED'
)`
