package leetcode

// LeetCode #610 - Triangle Judgement
//
// Three lengths form a triangle when each pair is longer than the remaining side.
const triangleJudgementSQL = `SELECT x, y, z,
    CASE
        WHEN x + y > z AND x + z > y AND y + z > x THEN 'Yes'
        ELSE 'No'
    END AS triangle
FROM Triangle`
