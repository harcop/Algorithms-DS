package leetcode

// LeetCode #41 - First Missing Positive
//
// Place each value v at index v-1, then return the first index whose value is not index+1.
func firstMissingPositive(nums []int) int {
	n := len(nums)
	for i := 0; i < n; i++ {
		for nums[i] > 0 && nums[i] <= n && nums[nums[i]-1] != nums[i] {
			idx := nums[i] - 1
			nums[i], nums[idx] = nums[idx], nums[i]
		}
	}

	for i, value := range nums {
		if value != i+1 {
			return i + 1
		}
	}
	return n + 1
}
