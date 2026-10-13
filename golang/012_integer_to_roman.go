package leetcode

import "strings"

// LeetCode #12 - Integer to Roman
//
// Greedy subtraction using the standard Roman numeral values, including the subtractive pairs.
func intToRoman(num int) string {
	values := []int{1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1}
	symbols := []string{"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"}

	var out strings.Builder
	for i, value := range values {
		for num >= value {
			num -= value
			out.WriteString(symbols[i])
		}
	}
	return out.String()
}
