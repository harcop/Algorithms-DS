package leetcode

// LeetCode #574 - Winning Candidate
//
// The winner is the candidate id that appears most often in Vote.
const winningCandidateSQL = `SELECT name
FROM Candidate
WHERE id = (
  SELECT candidateId
  FROM Vote
  GROUP BY candidateId
  ORDER BY COUNT(*) DESC
  LIMIT 1
)`
