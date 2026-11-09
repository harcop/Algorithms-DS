package leetcode

import "strconv"

// LeetCode #479 - Largest Palindrome Product
//
// Build palindromes from the upper half and keep the largest one that factors into two n-digit numbers.
func largestPalindrome(n int) int {
	if n == 1 {
		return 9
	}
	upper := int64(1)
	for i := 0; i < n; i++ {
		upper *= 10
	}
	upper--
	lower := (upper + 1) / 10
	for left := upper; left >= lower; left-- {
		text := strconv.FormatInt(left, 10)
		palindrome := text + reverseText(text)
		value, _ := strconv.ParseInt(palindrome, 10, 64)
		for factor := upper; factor*factor >= value; factor-- {
			if value%factor != 0 {
				continue
			}
			other := value / factor
			if other >= lower && other <= upper {
				return int(value % 1337)
			}
		}
	}
	return 0
}

func reverseText(s string) string {
	out := []byte(s)
	for i, j := 0, len(out)-1; i < j; i, j = i+1, j-1 {
		out[i], out[j] = out[j], out[i]
	}
	return string(out)
}
