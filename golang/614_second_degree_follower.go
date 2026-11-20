package leetcode

// LeetCode #614 - Second Degree Follower
//
// A person counts when they follow someone and at least one person follows them.
const secondDegreeFollowerSQL = `SELECT f1.follower, COUNT(DISTINCT f2.follower) AS num
FROM Follow f1
JOIN Follow f2 ON f1.follower = f2.followee
GROUP BY f1.follower
ORDER BY f1.follower`
