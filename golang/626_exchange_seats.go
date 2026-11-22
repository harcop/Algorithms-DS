package leetcode

// LeetCode #626 - Exchange Seats
//
// Odd seats trade with the next id, and the final odd seat stays when the count is odd.
const exchangeSeatsSQL = `SELECT
    CASE
        WHEN id % 2 = 1 AND id + 1 <= (SELECT MAX(id) FROM Seat) THEN id + 1
        WHEN id % 2 = 0 THEN id - 1
        ELSE id
    END AS id,
    student
FROM Seat
ORDER BY id`
