package leetcode

// LeetCode #175 - Combine Two Tables
//
// Every person stays in the result, with address columns left empty when none exists.
const combineTwoTablesSQL = `SELECT FirstName, LastName, City, State
FROM Person
LEFT JOIN Address ON Person.PersonId = Address.PersonId`
