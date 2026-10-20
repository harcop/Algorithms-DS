package leetcode

// LeetCode #178 - Rank Scores
//
// A score's rank is how many distinct scores are greater than or equal to it.
const rankScoresSQL = "SELECT s.Score,\n" +
	"  (SELECT COUNT(DISTINCT t.Score) FROM Scores t WHERE t.Score >= s.Score) AS `Rank`\n" +
	"FROM Scores s\n" +
	"ORDER BY s.Score DESC"
