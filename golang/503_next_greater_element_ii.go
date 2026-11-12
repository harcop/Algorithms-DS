package leetcode

// LeetCode #503 - Next Greater Element II
//
// A decreasing stack scans the circular array twice and writes the next greater value for each index.
func nextGreaterElements(nums []int) []int {
	n := len(nums)
	ans := make([]int, n)
	for i := range ans {
		ans[i] = -1
	}
	stack := make([]int, 0, n)
	for i := 0; i < 2*n; i++ {
		value := nums[i%n]
		for len(stack) > 0 && nums[stack[len(stack)-1]] < value {
			ans[stack[len(stack)-1]] = value
			stack = stack[:len(stack)-1]
		}
		if i < n {
			stack = append(stack, i)
		}
	}
	return ans
}
