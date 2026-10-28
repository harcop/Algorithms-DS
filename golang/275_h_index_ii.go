package leetcode

// LeetCode #275 - H-Index II
//
// LeetCode names this hIndex; that name already belongs to the unsorted version (#274).
// The citations are already sorted, so binary search for the largest h.
func hIndexII(citations []int) int {
	n := len(citations)
	lo, hi := 0, n
	for lo < hi {
		mid := lo + (hi-lo+1)/2
		if citations[n-mid] >= mid {
			lo = mid
		} else {
			hi = mid - 1
		}
	}
	return lo
}
