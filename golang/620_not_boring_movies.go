package leetcode

// LeetCode #620 - Not Boring Movies
//
// Odd ids that are not described as boring come back with the highest rating first.
const notBoringMoviesSQL = `SELECT id, movie, description, rating
FROM Cinema
WHERE id % 2 = 1 AND description != 'boring'
ORDER BY rating DESC`
