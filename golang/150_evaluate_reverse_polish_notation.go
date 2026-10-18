package leetcode

import "strconv"

// LeetCode #150 - Evaluate Reverse Polish Notation
//
// Numbers go on a stack. An operator replaces the top two numbers with their result.
func evalRPN(tokens []string) int {
	stack := make([]int, 0, len(tokens))
	for _, token := range tokens {
		switch token {
		case "+", "-", "*", "/":
			b := stack[len(stack)-1]
			a := stack[len(stack)-2]
			stack = stack[:len(stack)-2]
			switch token {
			case "+":
				stack = append(stack, a+b)
			case "-":
				stack = append(stack, a-b)
			case "*":
				stack = append(stack, a*b)
			case "/":
				stack = append(stack, a/b)
			}
		default:
			value, _ := strconv.Atoi(token)
			stack = append(stack, value)
		}
	}
	return stack[len(stack)-1]
}
