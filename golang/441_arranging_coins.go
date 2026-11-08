package leetcode

// LeetCode #441 - Arranging Coins
//
// Binary search the largest row whose triangular number still fits in the pile.
func arrangeCoins(n int) int {
	lo := int64(1)
	hi := int64(n)
	for lo <= hi {
		mid := lo + (hi-lo)/2
		need := mid * (mid + 1) / 2
		if need == int64(n) {
			return int(mid)
		}
		if need < int64(n) {
			lo = mid + 1
			continue
		}
		hi = mid - 1
	}
	return int(hi)
}
