package leetcode

// LeetCode #531 - Lonely Pixel I
//
// A black pixel is lonely when it is the only black pixel in its row and its column.
func findLonelyPixel(picture [][]byte) int {
	if len(picture) == 0 || len(picture[0]) == 0 {
		return 0
	}
	rows, cols := len(picture), len(picture[0])
	rowCount := make([]int, rows)
	colCount := make([]int, cols)
	for r := range picture {
		for c := range picture[r] {
			if picture[r][c] == 'B' {
				rowCount[r]++
				colCount[c]++
			}
		}
	}
	lonely := 0
	for r := range picture {
		for c := range picture[r] {
			if picture[r][c] == 'B' && rowCount[r] == 1 && colCount[c] == 1 {
				lonely++
			}
		}
	}
	return lonely
}
