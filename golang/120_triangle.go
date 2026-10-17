package leetcode

// LeetCode #120 - Triangle
//
// From the bottom, replace each entry with itself plus the cheaper adjacent entry below.
func minimumTotal(triangle [][]int) int {
	if len(triangle) == 0 {
		return 0
	}
	for i := len(triangle) - 2; i >= 0; i-- {
		for j := 0; j < len(triangle[i]); j++ {
			below := triangle[i+1][j]
			if triangle[i+1][j+1] < below {
				below = triangle[i+1][j+1]
			}
			triangle[i][j] += below
		}
	}
	return triangle[0][0]
}
