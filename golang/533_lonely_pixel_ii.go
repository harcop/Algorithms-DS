package leetcode

// LeetCode #533 - Lonely Pixel II
//
// LeetCode names this findLonelyPixel; that name already belongs to the single-black-pixel rule (#531).
// A black pixel counts when its row pattern and its column each contain exactly n black pixels.
func findLonelyPixelII(picture [][]byte, n int) int {
	if len(picture) == 0 || len(picture[0]) == 0 {
		return 0
	}
	rows, cols := len(picture), len(picture[0])
	rowPattern := make([]string, rows)
	rowBlack := make([]int, rows)
	colBlack := make([]int, cols)
	frequency := map[string]int{}
	for r := range picture {
		rowPattern[r] = string(picture[r])
		frequency[rowPattern[r]]++
		for c := range picture[r] {
			if picture[r][c] == 'B' {
				rowBlack[r]++
				colBlack[c]++
			}
		}
	}
	lonely := 0
	for r := range picture {
		if rowBlack[r] != n || frequency[rowPattern[r]] != n {
			continue
		}
		for c := range picture[r] {
			if picture[r][c] == 'B' && colBlack[c] == n {
				lonely++
			}
		}
	}
	return lonely
}
