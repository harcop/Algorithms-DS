package leetcode

// LeetCode #660 - Remove 9
//
// Numbers that never use the digit 9 are base 9, written back with decimal digits.
func newInteger(n int) int {
	out := 0
	place := 1
	for n > 0 {
		out += (n % 9) * place
		n /= 9
		place *= 10
	}
	return out
}
