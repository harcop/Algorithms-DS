package leetcode

// LeetCode #602 - Friend Requests II: Who Has the Most Friends
//
// Each accepted request adds one friend to both people, and the busiest person wins.
const mostFriendsSQL = `SELECT id, COUNT(*) AS num
FROM (
    SELECT requester_id AS id FROM RequestAccepted
    UNION ALL
    SELECT accepter_id AS id FROM RequestAccepted
) friends
GROUP BY id
ORDER BY num DESC
LIMIT 1`
