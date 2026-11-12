package leetcode

// LeetCode #511 - Game Play Analysis I
//
// Each player's first login is the earliest event date in Activity.
const gamePlayAnalysisISQL = `SELECT player_id, MIN(event_date) AS first_login
FROM Activity
GROUP BY player_id`
