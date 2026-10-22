package leetcode

// LeetCode #213 - House Robber II
//
// LeetCode names this rob; that name already belongs to the linear street (#198).
// The first and last houses touch, so rob one of the two open rows.
func robII(nums []int) int {
	n := len(nums)
	if n == 1 {
		return nums[0]
	}
	return max(robLinear(nums[:n-1]), robLinear(nums[1:]))
}

func robLinear(nums []int) int {
	prev, cur := 0, 0
	for _, value := range nums {
		prev, cur = cur, max(cur, prev+value)
	}
	return max(prev, cur)
}
