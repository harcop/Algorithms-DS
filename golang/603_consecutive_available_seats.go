package leetcode

// LeetCode #603 - Consecutive Available Seats
//
// A free seat counts when the seat beside it is free too.
const consecutiveSeatsSQL = `SELECT DISTINCT a.seat_id
FROM Cinema a
JOIN Cinema b ON ABS(a.seat_id - b.seat_id) = 1
WHERE a.free = 1 AND b.free = 1
ORDER BY a.seat_id`
