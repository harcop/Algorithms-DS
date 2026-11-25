package leetcode

// LeetCode #678 - Valid Parenthesis String
//
// The low and high balances track the fewest and most open parentheses a star could still leave.
func checkValidString(s string) bool {
	low, high := 0, 0
	for i := 0; i < len(s); i++ {
		switch s[i] {
		case '(':
			low++
			high++
		case ')':
			low--
			high--
		default:
			low--
			high++
		}
		if high < 0 {
			return false
		}
		if low < 0 {
			low = 0
		}
	}
	return low == 0
}
