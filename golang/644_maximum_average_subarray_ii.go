package leetcode

// LeetCode #644 - Maximum Average Subarray II
//
// Binary search the average, and a prefix minimum shows whether any window of length at least k can meet it.
func findMaxAverageII(nums []int, k int) float64 {
	lo, hi := float64(nums[0]), float64(nums[0])
	for _, num := range nums[1:] {
		value := float64(num)
		if value < lo {
			lo = value
		}
		if value > hi {
			hi = value
		}
	}
	for step := 0; step < 100; step++ {
		mid := (lo + hi) / 2
		if averageAtLeast(nums, k, mid) {
			lo = mid
		} else {
			hi = mid
		}
	}
	return lo
}

func averageAtLeast(nums []int, k int, target float64) bool {
	prefix := make([]float64, len(nums)+1)
	for i, num := range nums {
		prefix[i+1] = prefix[i] + float64(num) - target
	}
	minPrefix := 0.0
	for end := k; end <= len(nums); end++ {
		if prefix[end-k] < minPrefix {
			minPrefix = prefix[end-k]
		}
		if prefix[end] >= minPrefix-1e-7 {
			return true
		}
	}
	return false
}
