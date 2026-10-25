package leetcode

// LeetCode #256 - Paint House
//
// The cost of painting this house a color is that color plus the cheaper of the other two colors so far.
func minCost(costs [][]int) int {
	a, b, c := 0, 0, 0
	for _, row := range costs {
		a, b, c = row[0]+min(b, c), row[1]+min(a, c), row[2]+min(a, b)
	}
	return min(a, b, c)
}
