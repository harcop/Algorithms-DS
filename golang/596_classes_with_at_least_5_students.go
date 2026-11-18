package leetcode

// LeetCode #596 - Classes With at Least 5 Students
//
// Group the course rows by class and keep the groups of five or more.
const classesWithAtLeast5StudentsSQL = `SELECT class
FROM Courses
GROUP BY class
HAVING COUNT(*) >= 5`
