package leetcode

// LeetCode #22 - Generate Parentheses
//
// Backtrack, placing an open paren while any remain and a close paren only when it stays valid.
func generateParenthesis(n int) []string {
	var out []string
	current := make([]byte, 0, n*2)

	var backtrack func(open, close int)
	backtrack = func(open, close int) {
		if open == 0 && close == 0 {
			out = append(out, string(current))
			return
		}
		if open > 0 {
			current = append(current, '(')
			backtrack(open-1, close)
			current = current[:len(current)-1]
		}
		if close > open {
			current = append(current, ')')
			backtrack(open, close-1)
			current = current[:len(current)-1]
		}
	}

	backtrack(n, n)
	return out
}
