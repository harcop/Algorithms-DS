package leetcode

// LeetCode #688 - Knight Probability in Chessboard
//
// Each move spreads a square's probability across the eight knight jumps that stay on the board.
func knightProbability(n, k, row, column int) float64 {
	dirs := [][2]int{{-2, -1}, {-2, 1}, {-1, -2}, {-1, 2}, {1, -2}, {1, 2}, {2, -1}, {2, 1}}
	prob := make([][]float64, n)
	for r := range prob {
		prob[r] = make([]float64, n)
	}
	prob[row][column] = 1
	for step := 0; step < k; step++ {
		next := make([][]float64, n)
		for r := range next {
			next[r] = make([]float64, n)
		}
		for r := 0; r < n; r++ {
			for c := 0; c < n; c++ {
				if prob[r][c] == 0 {
					continue
				}
				for _, dir := range dirs {
					nr, nc := r+dir[0], c+dir[1]
					if nr < 0 || nc < 0 || nr >= n || nc >= n {
						continue
					}
					next[nr][nc] += prob[r][c] / 8
				}
			}
		}
		prob = next
	}
	total := 0.0
	for r := range prob {
		for _, value := range prob[r] {
			total += value
		}
	}
	return total
}
