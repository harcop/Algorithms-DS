package leetcode

import (
	"math"
	"strconv"
)

// LeetCode #556 - Next Greater Element III
//
// LeetCode names this nextGreaterElement; that name already belongs to the two-array version (#496).
// Swap the first descent with the next larger digit on its right, then sort the tail upward.
func nextGreaterElementIII(n int) int {
	digits := []byte(strconv.Itoa(n))
	pivot := len(digits) - 2
	for pivot >= 0 && digits[pivot] >= digits[pivot+1] {
		pivot--
	}
	if pivot < 0 {
		return -1
	}
	swap := len(digits) - 1
	for digits[swap] <= digits[pivot] {
		swap--
	}
	digits[pivot], digits[swap] = digits[swap], digits[pivot]
	for left, right := pivot+1, len(digits)-1; left < right; left, right = left+1, right-1 {
		digits[left], digits[right] = digits[right], digits[left]
	}
	value, err := strconv.ParseInt(string(digits), 10, 64)
	if err != nil || value > math.MaxInt32 || int(value) <= n {
		return -1
	}
	return int(value)
}
