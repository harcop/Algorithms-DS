package leetcode

import "sort"

// LeetCode #296 - Best Meeting Point
//
// The best row and column are the medians of the occupied rows and columns.
func minTotalDistance(grid [][]int) int {
	var rows, cols []int
	for i := range grid {
		for j := range grid[i] {
			if grid[i][j] == 1 {
				rows = append(rows, i)
				cols = append(cols, j)
			}
		}
	}
	sort.Ints(rows)
	sort.Ints(cols)
	meetRow, meetCol := rows[len(rows)/2], cols[len(cols)/2]
	distance := 0
	for _, row := range rows {
		distance += absInt(row - meetRow)
	}
	for _, col := range cols {
		distance += absInt(col - meetCol)
	}
	return distance
}
