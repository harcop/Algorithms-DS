package leetcode

// LeetCode #595 - Big Countries
//
// A country is big when its area or its population crosses the given line.
const bigCountriesSQL = `SELECT name, population, area
FROM World
WHERE area >= 3000000 OR population >= 25000000`
