package leetcode

// LeetCode #439 - Ternary Expression Parser
//
// Scanning from the right resolves each choice before the condition that selects it.
func parseTernary(expression string) string {
	stack := make([]byte, 0, len(expression))
	for i := len(expression) - 1; i >= 0; i-- {
		if expression[i] == '?' {
			trueVal := stack[len(stack)-1]
			falseVal := stack[len(stack)-2]
			stack = stack[:len(stack)-2]
			i--
			chosen := falseVal
			if expression[i] == 'T' {
				chosen = trueVal
			}
			stack = append(stack, chosen)
			continue
		}
		if expression[i] != ':' {
			stack = append(stack, expression[i])
		}
	}
	return string(stack[len(stack)-1:])
}
