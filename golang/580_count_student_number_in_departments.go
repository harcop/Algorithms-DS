package leetcode

// LeetCode #580 - Count Student Number in Departments
//
// A left join keeps departments that have no students, and the count is ordered high to low.
const studentCountByDepartmentSQL = `SELECT d.dept_name, COUNT(s.student_id) AS student_number
FROM Department d
LEFT JOIN Student s ON d.dept_id = s.dept_id
GROUP BY d.dept_id, d.dept_name
ORDER BY student_number DESC, d.dept_name`
