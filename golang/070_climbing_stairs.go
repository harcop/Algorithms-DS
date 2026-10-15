package leetcode

// LeetCode #70 - Climbing Stairs
//
// The ways to reach step n are the ways to reach n-1 plus the ways to reach n-2.
func climbStairs(n int) int {
	if n <= 2 {
		return n
	}
	a, b := 1, 2
	for step := 3; step <= n; step++ {
		a, b = b, a+b
	}
	return b
}
