package leetcode

// LeetCode #410 - Split Array Largest Sum
//
// Binary search the smallest cap that still lets the array be cut into k contiguous pieces.
func splitArray(nums []int, k int) int {
	lo := int64(0)
	hi := int64(0)
	for _, num := range nums {
		value := int64(num)
		if value > lo {
			lo = value
		}
		hi += value
	}
	for lo < hi {
		mid := lo + (hi-lo)/2
		if splitFits(nums, mid, k) {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return int(lo)
}

func splitFits(nums []int, cap int64, k int) bool {
	parts := 1
	cur := int64(0)
	for _, num := range nums {
		value := int64(num)
		if cur+value > cap {
			parts++
			cur = value
			if parts > k {
				return false
			}
			continue
		}
		cur += value
	}
	return true
}
