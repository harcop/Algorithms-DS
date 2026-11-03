package leetcode

// LeetCode #390 - Elimination Game
//
// Track the first remaining number while the step doubles and the direction flips.
func lastRemaining(n int) int {
	head := 1
	step := 1
	left := true
	for n > 1 {
		if left || n%2 == 1 {
			head += step
		}
		n /= 2
		step *= 2
		left = !left
	}
	return head
}
