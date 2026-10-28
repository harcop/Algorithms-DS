package leetcode

// LeetCode #276 - Paint Fence
//
// A post matches the previous color or uses a different one, and three matches in a row are forbidden.
func numWays(n int, k int) int {
	if n == 0 {
		return 0
	}
	if n == 1 {
		return k
	}
	same, diff := k, k*(k-1)
	for i := 2; i < n; i++ {
		same, diff = diff, (same+diff)*(k-1)
	}
	return same + diff
}
