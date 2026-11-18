package leetcode

// LeetCode #598 - Range Addition II
//
// Every operation covers the top-left corner, so the maximum is the overlap of all of them.
func maxCount(m int, n int, ops [][]int) int {
	for _, op := range ops {
		m = min(m, op[0])
		n = min(n, op[1])
	}
	return m * n
}
