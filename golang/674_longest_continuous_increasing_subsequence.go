package leetcode

// LeetCode #674 - Longest Continuous Increasing Subsequence
//
// A rise continues the current run, and any other step starts a new run of one.
func findLengthOfLCIS(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	best, run := 1, 1
	for i := 1; i < len(nums); i++ {
		if nums[i] > nums[i-1] {
			run++
			best = max(best, run)
			continue
		}
		run = 1
	}
	return best
}
