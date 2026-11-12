package leetcode

// LeetCode #504 - Base 7
//
// Repeated division by 7 writes the digits from least significant to most significant.
func convertToBase7(num int) string {
	if num == 0 {
		return "0"
	}
	negative := num < 0
	if negative {
		num = -num
	}
	digits := make([]byte, 0, 12)
	for num > 0 {
		digits = append(digits, byte('0'+num%7))
		num /= 7
	}
	if negative {
		digits = append(digits, '-')
	}
	for i, j := 0, len(digits)-1; i < j; i, j = i+1, j-1 {
		digits[i], digits[j] = digits[j], digits[i]
	}
	return string(digits)
}
