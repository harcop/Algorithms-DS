package leetcode

// LeetCode #571 - Find Median Given Frequency of Numbers
//
// A value is a median when its frequency still covers the gap between the counts on either side.
const medianFrequencySQL = `SELECT AVG(num) AS median
FROM Numbers n
WHERE n.frequency >= ABS(
  (SELECT SUM(frequency) FROM Numbers WHERE num <= n.num) -
  (SELECT SUM(frequency) FROM Numbers WHERE num >= n.num)
)`
