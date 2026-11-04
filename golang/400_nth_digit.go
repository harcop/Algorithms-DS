package leetcode

import "strconv"

// LeetCode #400 - Nth Digit
//
// Skip whole blocks of 1-digit, 2-digit, and longer numbers, then read the digit inside the last number.
func findNthDigit(n int) int {
	remaining := int64(n)
	length := int64(1)
	count := int64(9)
	start := int64(1)
	for remaining > length*count {
		remaining -= length * count
		length++
		count *= 10
		start *= 10
	}
	number := start + (remaining-1)/length
	index := (remaining - 1) % length
	text := strconv.FormatInt(number, 10)
	return int(text[index] - '0')
}
