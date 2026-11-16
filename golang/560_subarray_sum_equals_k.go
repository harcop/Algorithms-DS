package leetcode

// LeetCode #560 - Subarray Sum Equals K
//
// A prefix sum that has already appeared k below the current sum closes one more subarray.
func subarraySum(nums []int, k int) int {
	seen := map[int]int{0: 1}
	sum, count := 0, 0
	for _, value := range nums {
		sum += value
		count += seen[sum-k]
		seen[sum]++
	}
	return count
}
