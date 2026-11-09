package leetcode

// LeetCode #476 - Number Complement
//
// Flip every bit of the number and keep only the bits that its original width used.
func findComplement(num int) int {
	if num == 0 {
		return 1
	}
	mask := 0
	for value := num; value > 0; value >>= 1 {
		mask = (mask << 1) | 1
	}
	return ^num & mask
}
