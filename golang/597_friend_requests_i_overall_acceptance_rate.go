package leetcode

// LeetCode #597 - Friend Requests I: Overall Acceptance Rate
//
// Distinct accepted pairs divided by distinct sent pairs, or zero when nobody sent a request.
const friendAcceptanceRateSQL = `SELECT ROUND(IFNULL(
  (SELECT COUNT(*) FROM (SELECT DISTINCT requester_id, accepter_id FROM RequestAccepted) accepted) /
  (SELECT COUNT(*) FROM (SELECT DISTINCT sender_id, send_to_id FROM FriendRequest) sent),
  0
), 2) AS accept_rate`
