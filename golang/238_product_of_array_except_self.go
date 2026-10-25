package leetcode

// LeetCode #238 - Product of Array Except Self
//
// The prefix product and the suffix product meet at each index.
func productExceptSelf(nums []int) []int {
	out := make([]int, len(nums))
	product := 1
	for i, value := range nums {
		out[i] = product
		product *= value
	}
	product = 1
	for i := len(nums) - 1; i >= 0; i-- {
		out[i] *= product
		product *= nums[i]
	}
	return out
}
