package leetcode

// LeetCode #629 - K Inverse Pairs Array
//
// The next number adds anywhere from 0 to n-1 new inverse pairs, so a window sums those choices.
func kInversePairs(n int, k int) int {
	const mod = 1_000_000_007
	ways := make([]int, k+1)
	ways[0] = 1
	for length := 2; length <= n; length++ {
		next := make([]int, k+1)
		window := 0
		for pairs := 0; pairs <= k; pairs++ {
			window += ways[pairs]
			if window >= mod {
				window -= mod
			}
			if pairs >= length {
				window -= ways[pairs-length]
				if window < 0 {
					window += mod
				}
			}
			next[pairs] = window
		}
		ways = next
	}
	return ways[k]
}
