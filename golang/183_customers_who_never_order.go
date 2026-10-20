package leetcode

// LeetCode #183 - Customers Who Never Order
//
// A left join with a null order id is a customer who never ordered.
const customersWhoNeverOrderSQL = `SELECT Name AS Customers
FROM Customers c
LEFT JOIN Orders o ON c.Id = o.CustomerId
WHERE o.Id IS NULL`
