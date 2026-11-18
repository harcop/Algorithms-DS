package leetcode

// LeetCode #586 - Customer Placing the Largest Number of Orders
//
// Group the orders by customer and keep the one with the most rows.
const largestNumberOfOrdersSQL = `SELECT customer_number
FROM Orders
GROUP BY customer_number
ORDER BY COUNT(*) DESC
LIMIT 1`
