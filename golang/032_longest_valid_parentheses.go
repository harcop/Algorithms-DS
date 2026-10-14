package leetcode

// LeetCode #32 - Longest Valid Parentheses
//
// Keep indexes of unmatched openers. A closer either extends a valid span or becomes the new base.
func longestValidParentheses(s string) int {
	stack := []int{-1}
	best := 0

	for i := 0; i < len(s); i++ {
		if s[i] == '(' {
			stack = append(stack, i)
			continue
		}

		stack = stack[:len(stack)-1]
		if len(stack) == 0 {
			stack = append(stack, i)
			continue
		}
		if span := i - stack[len(stack)-1]; span > best {
			best = span
		}
	}

	return best
}
