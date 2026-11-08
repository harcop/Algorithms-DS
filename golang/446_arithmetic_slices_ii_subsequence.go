package leetcode

// LeetCode #446 - Arithmetic Slices II - Subsequence
//
// LeetCode names this numberOfArithmeticSlices; that name already belongs to the contiguous slices (#413).
// Each pair extends every shorter subsequence that already ends with the same difference.
func numberOfArithmeticSlicesII(nums []int) int {
	if len(nums) < 3 {
		return 0
	}
	dp := make([]map[int64]int, len(nums))
	for i := range dp {
		dp[i] = map[int64]int{}
	}
	total := 0
	for i := 0; i < len(nums); i++ {
		for j := 0; j < i; j++ {
			diff := int64(nums[i]) - int64(nums[j])
			count := dp[j][diff] + 1
			dp[i][diff] += count
			total += count - 1
		}
	}
	return total
}
