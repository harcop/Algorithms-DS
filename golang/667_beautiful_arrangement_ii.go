package leetcode

// LeetCode #667 - Beautiful Arrangement II
//
// Alternating the low and high ends of 1..k+1 produces k distinct gaps, and the rest follow in order.
func constructArray(n, k int) []int {
	out := make([]int, 0, n)
	low, high := 1, k+1
	for low <= high {
		out = append(out, low)
		low++
		if low <= high {
			out = append(out, high)
			high--
		}
	}
	for value := k + 2; value <= n; value++ {
		out = append(out, value)
	}
	return out
}
