package leetcode

import "strconv"

// LeetCode #241 - Different Ways to Add Parentheses
//
// Split on each operator and combine every value from the left piece with every value from the right.
func diffWaysToCompute(expression string) []int {
	var out []int
	for i := 0; i < len(expression); i++ {
		op := expression[i]
		if op != '+' && op != '-' && op != '*' {
			continue
		}
		left := diffWaysToCompute(expression[:i])
		right := diffWaysToCompute(expression[i+1:])
		for _, a := range left {
			for _, b := range right {
				switch op {
				case '+':
					out = append(out, a+b)
				case '-':
					out = append(out, a-b)
				default:
					out = append(out, a*b)
				}
			}
		}
	}
	if len(out) == 0 {
		value, _ := strconv.Atoi(expression)
		return []int{value}
	}
	return out
}
