package leetcode

// LeetCode #619 - Biggest Single Number
//
// Numbers that appear once are eligible, and the largest of them is the answer.
const biggestSingleNumberSQL = `SELECT MAX(num) AS num
FROM (
    SELECT num
    FROM MyNumbers
    GROUP BY num
    HAVING COUNT(*) = 1
) singles`
