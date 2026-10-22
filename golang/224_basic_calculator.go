package leetcode

// LeetCode #224 - Basic Calculator
//
// A sign walks with the next number, and parentheses start a nested sum.
func calculate(s string) int {
	i := 0
	var expr func() int
	expr = func() int {
		sign, sum := 1, 0
		for i < len(s) {
			switch s[i] {
			case ' ':
				i++
			case '+':
				i++
				sign = 1
			case '-':
				i++
				sign = -1
			case '(':
				i++
				sum += sign * expr()
				sign = 1
			case ')':
				i++
				return sum
			default:
				n := 0
				for i < len(s) && s[i] >= '0' && s[i] <= '9' {
					n = n*10 + int(s[i]-'0')
					i++
				}
				sum += sign * n
				sign = 1
			}
		}
		return sum
	}
	return expr()
}
