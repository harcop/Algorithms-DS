package leetcode

// LeetCode #565 - Array Nesting
//
// The permutation is a set of cycles, and the answer is the longest cycle.
func arrayNesting(nums []int) int {
	seen := make([]bool, len(nums))
	best := 0
	for start := range nums {
		length := 0
		for index := start; !seen[index]; index = nums[index] {
			seen[index] = true
			length++
		}
		best = max(best, length)
	}
	return best
}
