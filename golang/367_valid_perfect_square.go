package leetcode

// LeetCode #367 - Valid Perfect Square
//
// Binary search for an integer whose square equals the number.
func isPerfectSquare(num int) bool {
	if num < 1 {
		return false
	}
	lo, hi := 1, num
	target := int64(num)
	for lo <= hi {
		mid := lo + (hi-lo)/2
		square := int64(mid) * int64(mid)
		if square == target {
			return true
		}
		if square < target {
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return false
}
