package leetcode

// LeetCode #618 - Students Report By Geography
//
// Rank names inside each continent, then line those ranks up under America, Asia, and Europe.
const studentsByGeographySQL = `SELECT
    MAX(CASE WHEN continent = 'America' THEN name END) AS America,
    MAX(CASE WHEN continent = 'Asia' THEN name END) AS Asia,
    MAX(CASE WHEN continent = 'Europe' THEN name END) AS Europe
FROM (
    SELECT name, continent,
           ROW_NUMBER() OVER (PARTITION BY continent ORDER BY name) AS rk
    FROM Student
) ranked
GROUP BY rk`
