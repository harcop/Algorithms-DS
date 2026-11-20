package leetcode

// LeetCode #615 - Average Salary: Departments VS Company
//
// Each month, a department average sits above, below, or level with the company average.
const averageSalaryComparisonSQL = `SELECT dept.pay_month, dept.department_id,
    CASE
        WHEN dept.department_avg > company.company_avg THEN 'higher'
        WHEN dept.department_avg < company.company_avg THEN 'lower'
        ELSE 'same'
    END AS comparison
FROM (
    SELECT e.department_id,
           AVG(s.amount) AS department_avg,
           DATE_FORMAT(s.pay_date, '%Y-%m') AS pay_month
    FROM Salary s
    JOIN Employee e ON s.employee_id = e.employee_id
    GROUP BY e.department_id, DATE_FORMAT(s.pay_date, '%Y-%m')
) dept
JOIN (
    SELECT AVG(amount) AS company_avg, DATE_FORMAT(pay_date, '%Y-%m') AS pay_month
    FROM Salary
    GROUP BY DATE_FORMAT(pay_date, '%Y-%m')
) company ON dept.pay_month = company.pay_month`
