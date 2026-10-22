package leetcode

// LeetCode #227 - Basic Calculator II
//
// LeetCode names this calculate; that name already belongs to the calculator with parentheses (#224).
// Apply * and / as they appear, and add the signed terms at the end.
func calculateII(s string) int {
	var terms []int
	num := 0
	op := byte('+')
	for i := 0; i < len(s); i++ {
		ch := s[i]
		if ch >= '0' && ch <= '9' {
			num = num*10 + int(ch-'0')
		}
		if ((ch < '0' || ch > '9') && ch != ' ') || i+1 == len(s) {
			switch op {
			case '+':
				terms = append(terms, num)
			case '-':
				terms = append(terms, -num)
			case '*':
				terms[len(terms)-1] *= num
			case '/':
				terms[len(terms)-1] /= num
			}
			if ch == '+' || ch == '-' || ch == '*' || ch == '/' {
				op = ch
			}
			num = 0
		}
	}
	sum := 0
	for _, term := range terms {
		sum += term
	}
	return sum
}
