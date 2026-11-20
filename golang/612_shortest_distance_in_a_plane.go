package leetcode

// LeetCode #612 - Shortest Distance in a Plane
//
// Pair each point with a later one and keep the smallest Euclidean distance, rounded to two places.
const shortestDistancePlaneSQL = `SELECT ROUND(SQRT(MIN(POW(p1.x - p2.x, 2) + POW(p1.y - p2.y, 2))), 2) AS shortest
FROM Point2D p1
JOIN Point2D p2 ON p1.x < p2.x OR (p1.x = p2.x AND p1.y < p2.y)`
