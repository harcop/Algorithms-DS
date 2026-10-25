package leetcode

// LeetCode #268 - Missing Number
//
// The missing value is the gap between the triangular sum and the sum of the array.
func missingNumber(nums []int) int {
	n := len(nums)
	sum := 0
	for _, value := range nums {
		sum += value
	}
	return n*(n+1)/2 - sum
}
