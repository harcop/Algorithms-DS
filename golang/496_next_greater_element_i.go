package leetcode

// LeetCode #496 - Next Greater Element I
//
// A decreasing stack records the next greater value in the second array, then the first array reads those answers.
func nextGreaterElement(nums1 []int, nums2 []int) []int {
	next := map[int]int{}
	stack := make([]int, 0, len(nums2))
	for _, value := range nums2 {
		for len(stack) > 0 && stack[len(stack)-1] < value {
			next[stack[len(stack)-1]] = value
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, value)
	}
	out := make([]int, len(nums1))
	for i, value := range nums1 {
		if greater, ok := next[value]; ok {
			out[i] = greater
			continue
		}
		out[i] = -1
	}
	return out
}
