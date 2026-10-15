package leetcode

// LeetCode #55 - Jump Game
//
// Track the farthest index reachable so far and stop when a gap appears beyond it.
func canJump(nums []int) bool {
	farthest := 0
	for i, step := range nums {
		if i > farthest {
			return false
		}
		if next := i + step; next > farthest {
			farthest = next
		}
		if farthest >= len(nums)-1 {
			return true
		}
	}
	return true
}
