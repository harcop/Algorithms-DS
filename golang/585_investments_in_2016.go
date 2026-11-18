package leetcode

// LeetCode #585 - Investments in 2016
//
// Keep a 2015 amount shared with someone else, and a city that nobody else occupies.
const investmentsIn2016SQL = `SELECT ROUND(SUM(i.tiv_2016), 2) AS tiv_2016
FROM Insurance i
WHERE i.tiv_2015 IN (
  SELECT tiv_2015 FROM Insurance GROUP BY tiv_2015 HAVING COUNT(*) > 1
)
AND NOT EXISTS (
  SELECT 1 FROM Insurance other
  WHERE other.lat = i.lat AND other.lon = i.lon AND other.pid != i.pid
)`
