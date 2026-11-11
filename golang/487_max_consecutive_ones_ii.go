package leetcode

// LeetCode #487 - Max Consecutive Ones II
//
// LeetCode names this findMaxConsecutiveOnes; that name already belongs to the version with no flips (#485).
// A window may contain one zero, and it shrinks until that is true again.
func findMaxConsecutiveOnesII(nums []int) int {
	left, zeros, best := 0, 0, 0
	for right, value := range nums {
		if value == 0 {
			zeros++
		}
		for zeros > 1 {
			if nums[left] == 0 {
				zeros--
			}
			left++
		}
		best = max(best, right-left+1)
	}
	return best
}
