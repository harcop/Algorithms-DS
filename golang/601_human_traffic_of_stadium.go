package leetcode

// LeetCode #601 - Human Traffic of Stadium
//
// Rows with at least 100 people form a streak when id minus its rank stays constant.
const humanTrafficSQL = `WITH crowded AS (
    SELECT id, visit_date, people,
           id - ROW_NUMBER() OVER (ORDER BY id) AS grp
    FROM Stadium
    WHERE people >= 100
)
SELECT id, visit_date, people
FROM crowded
WHERE grp IN (
    SELECT grp FROM crowded GROUP BY grp HAVING COUNT(*) >= 3
)
ORDER BY visit_date`
