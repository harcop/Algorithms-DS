package leetcode

// LeetCode #534 - Game Play Analysis III
//
// A running sum of games played, ordered by date, is the total so far for each player.
const gamePlayAnalysisIIISQL = `SELECT player_id, event_date,
       SUM(games_played) OVER (PARTITION BY player_id ORDER BY event_date) AS games_played_so_far
FROM Activity`
