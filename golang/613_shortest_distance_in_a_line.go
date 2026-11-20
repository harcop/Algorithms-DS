package leetcode

// LeetCode #613 - Shortest Distance in a Line
//
// The closest pair of distinct x values is the minimum absolute gap.
const shortestDistanceLineSQL = `SELECT MIN(ABS(p1.x - p2.x)) AS shortest
FROM Point p1
JOIN Point p2 ON p1.x < p2.x`
