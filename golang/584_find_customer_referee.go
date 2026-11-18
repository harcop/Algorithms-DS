package leetcode

// LeetCode #584 - Find Customer Referee
//
// A missing referee is not referee 2, so the null check has to stay.
const findCustomerRefereeSQL = `SELECT name
FROM Customer
WHERE referee_id != 2 OR referee_id IS NULL`
