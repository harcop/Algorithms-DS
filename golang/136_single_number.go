package leetcode

// LeetCode #136 - Single Number
//
// XOR cancels every value that appears twice and leaves the value that appears once.
func singleNumber(nums []int) int {
	result := 0
	for _, num := range nums {
		result ^= num
	}
	return result
}
