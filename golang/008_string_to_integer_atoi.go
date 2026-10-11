package leetcode

import "math"

// LeetCode #8 - String to Integer (atoi)
//
// Implement myAtoi: whitespace, optional sign, digits, clamp to 32-bit range.
func myAtoi(s string) int {
	i := 0
	for i < len(s) && s[i] == ' ' {
		i++
	}
	if i == len(s) {
		return 0
	}

	sign := 1
	switch s[i] {
	case '+':
		i++
	case '-':
		sign = -1
		i++
	}

	acc := 0
	for i < len(s) && s[i] >= '0' && s[i] <= '9' {
		digit := int(s[i] - '0')
		if acc > (math.MaxInt32-digit)/10 {
			if sign == 1 {
				return math.MaxInt32
			}
			return math.MinInt32
		}
		acc = acc*10 + digit
		i++
	}

	return sign * acc
}
