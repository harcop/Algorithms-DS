package leetcode

// LeetCode #668 - Kth Smallest Number in Multiplication Table
//
// Binary search the value, counting how many products i*j are no larger than the guess.
func kthSmallestInTable(m, n, k int) int {
	if m > n {
		m, n = n, m
	}
	lo, hi := 1, m*n
	for lo < hi {
		mid := (lo + hi) / 2
		if productsNotGreater(m, n, mid) < k {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return lo
}

func productsNotGreater(m, n, limit int) int {
	count := 0
	for row := 1; row <= m; row++ {
		count += min(n, limit/row)
	}
	return count
}
