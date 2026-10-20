package leetcode

// LeetCode #182 - Duplicate Emails
//
// Group by email and keep the groups that contain more than one row.
const duplicateEmailsSQL = `SELECT Email
FROM Person
GROUP BY Email
HAVING COUNT(*) > 1`
