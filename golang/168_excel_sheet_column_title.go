package leetcode

// LeetCode #168 - Excel Sheet Column Title
//
// Base 26 with no zero: subtract one before each remainder so 1 maps to A.
func convertToTitle(columnNumber int) string {
	var digits []byte
	for columnNumber > 0 {
		columnNumber--
		digits = append(digits, byte('A'+columnNumber%26))
		columnNumber /= 26
	}
	for i, j := 0, len(digits)-1; i < j; i, j = i+1, j-1 {
		digits[i], digits[j] = digits[j], digits[i]
	}
	return string(digits)
}
