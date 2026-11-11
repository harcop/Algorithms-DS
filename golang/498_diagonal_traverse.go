package leetcode

// LeetCode #498 - Diagonal Traverse
//
// Walk up-right until a border, then down-left, turning at each edge.
func findDiagonalOrder(mat [][]int) []int {
	if len(mat) == 0 || len(mat[0]) == 0 {
		return nil
	}
	rows, cols := len(mat), len(mat[0])
	out := make([]int, 0, rows*cols)
	row, col := 0, 0
	up := true
	for len(out) < rows*cols {
		out = append(out, mat[row][col])
		if up {
			if col == cols-1 {
				row++
				up = false
			} else if row == 0 {
				col++
				up = false
			} else {
				row--
				col++
			}
			continue
		}
		if row == rows-1 {
			col++
			up = true
			continue
		}
		if col == 0 {
			row++
			up = true
			continue
		}
		row++
		col--
	}
	return out
}
