package leetcode

// LeetCode #132 - Palindrome Partitioning II
//
// Cuts needed for a prefix is one more than the cuts before a palindrome that ends here.
func minCut(s string) int {
	n := len(s)
	if n <= 1 {
		return 0
	}
	pal := make([][]bool, n)
	for i := range pal {
		pal[i] = make([]bool, n)
	}
	for i := n - 1; i >= 0; i-- {
		for j := i; j < n; j++ {
			pal[i][j] = s[i] == s[j] && (j-i < 2 || pal[i+1][j-1])
		}
	}

	cuts := make([]int, n)
	for end := 0; end < n; end++ {
		best := end + 1
		for start := 0; start <= end; start++ {
			if !pal[start][end] {
				continue
			}
			if start == 0 {
				best = 0
				continue
			}
			if next := cuts[start-1] + 1; next < best {
				best = next
			}
		}
		cuts[end] = best
	}
	return cuts[n-1]
}
