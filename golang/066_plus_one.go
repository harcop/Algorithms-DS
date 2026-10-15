package leetcode

// LeetCode #66 - Plus One
//
// Add one from the right and carry a new leading digit when every digit was 9.
func plusOne(digits []int) []int {
	carry := 1
	for i := len(digits) - 1; i >= 0 && carry > 0; i-- {
		sum := digits[i] + carry
		digits[i] = sum % 10
		carry = sum / 10
	}
	if carry > 0 {
		digits = append([]int{carry}, digits...)
	}
	return digits
}
