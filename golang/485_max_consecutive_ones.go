package leetcode

// LeetCode #485 - Max Consecutive Ones
//
// Walk the row once and keep the longest streak of ones.
func findMaxConsecutiveOnes(nums []int) int {
	best, current := 0, 0
	for _, value := range nums {
		if value == 1 {
			current++
			best = max(best, current)
			continue
		}
		current = 0
	}
	return best
}
