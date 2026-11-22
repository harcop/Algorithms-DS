package leetcode

// LeetCode #634 - Find the Derangement of An Array
//
// A derangement of n items is built from the derangements of n-1 and n-2.
func findDerangement(n int) int {
	const mod = 1_000_000_007
	if n <= 1 {
		return 0
	}
	prev2, prev1 := 1, 0
	for i := 2; i <= n; i++ {
		prev2, prev1 = prev1, int((int64(i-1)*int64(prev2+prev1))%mod)
	}
	return prev1
}
