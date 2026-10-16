package leetcode

// LeetCode #89 - Gray Code
//
// Each index reflected once, i ^ (i >> 1), differs from the next code by a single bit.
func grayCode(n int) []int {
	length := 1 << n
	out := make([]int, length)
	for i := 0; i < length; i++ {
		out[i] = i ^ (i >> 1)
	}
	return out
}
