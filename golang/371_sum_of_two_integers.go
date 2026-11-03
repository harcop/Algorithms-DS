package leetcode

// LeetCode #371 - Sum of Two Integers
//
// Add with XOR and shift the carry until nothing remains.
func getSum(a, b int) int {
	x := uint32(a)
	y := uint32(b)
	for y != 0 {
		carry := (x & y) << 1
		x ^= y
		y = carry
	}
	return int(int32(x))
}
