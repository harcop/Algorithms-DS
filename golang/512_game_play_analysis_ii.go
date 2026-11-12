package leetcode

// LeetCode #512 - Game Play Analysis II
//
// The device used on a player's earliest event date is the one from their first login.
const gamePlayAnalysisIISQL = `SELECT a.player_id, a.device_id
FROM Activity a
JOIN (
  SELECT player_id, MIN(event_date) AS first_login
  FROM Activity
  GROUP BY player_id
) first ON a.player_id = first.player_id AND a.event_date = first.first_login`
