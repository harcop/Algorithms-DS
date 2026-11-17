package leetcode

// LeetCode #578 - Get Highest Answer Rate Question
//
// Answer rate is answers divided by shows, and the smallest question id breaks a tie.
const highestAnswerRateSQL = `SELECT question_id AS survey_log
FROM SurveyLog
GROUP BY question_id
ORDER BY SUM(action = 'answer') / SUM(action = 'show') DESC, question_id
LIMIT 1`
