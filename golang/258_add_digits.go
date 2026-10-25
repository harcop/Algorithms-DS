package leetcode

// LeetCode #258 - Add Digits
//
// The repeated digit sum is the number modulo 9, except multiples of 9 stay at 9.
func addDigits(num int) int {
	if num == 0 {
		return 0
	}
	if num%9 == 0 {
		return 9
	}
	return num % 9
}
