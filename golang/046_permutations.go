package leetcode

// LeetCode #46 - Permutations
//
// Swap each unused value into the current position, then recurse on the suffix.
func permute(nums []int) [][]int {
	var result [][]int

	var dfs func(start int)
	dfs = func(start int) {
		if start == len(nums) {
			result = append(result, append([]int(nil), nums...))
			return
		}
		for i := start; i < len(nums); i++ {
			nums[start], nums[i] = nums[i], nums[start]
			dfs(start + 1)
			nums[start], nums[i] = nums[i], nums[start]
		}
	}

	dfs(0)
	return result
}
