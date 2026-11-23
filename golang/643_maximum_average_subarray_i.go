package leetcode

// LeetCode #643 - Maximum Average Subarray I
//
// A window of length k slides across the array, and the largest window sum has the largest average.
func findMaxAverage(nums []int, k int) float64 {
	sum := 0
	for i := 0; i < k; i++ {
		sum += nums[i]
	}
	best := sum
	for i := k; i < len(nums); i++ {
		sum += nums[i] - nums[i-k]
		if sum > best {
			best = sum
		}
	}
	return float64(best) / float64(k)
}
