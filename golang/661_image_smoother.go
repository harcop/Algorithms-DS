package leetcode

// LeetCode #661 - Image Smoother
//
// Each cell becomes the floor of the average of itself and the neighbors that exist.
func imageSmoother(img [][]int) [][]int {
	rows, cols := len(img), len(img[0])
	out := make([][]int, rows)
	for r := 0; r < rows; r++ {
		out[r] = make([]int, cols)
		for c := 0; c < cols; c++ {
			sum, count := 0, 0
			for dr := -1; dr <= 1; dr++ {
				for dc := -1; dc <= 1; dc++ {
					nr, nc := r+dr, c+dc
					if nr < 0 || nc < 0 || nr >= rows || nc >= cols {
						continue
					}
					sum += img[nr][nc]
					count++
				}
			}
			out[r][c] = sum / count
		}
	}
	return out
}
