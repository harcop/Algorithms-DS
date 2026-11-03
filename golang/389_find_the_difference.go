package leetcode

// LeetCode #389 - Find the Difference
//
// XOR every character in both strings; the extra character is what remains.
func findTheDifference(s, t string) byte {
	var diff byte
	for i := 0; i < len(s); i++ {
		diff ^= s[i]
	}
	for i := 0; i < len(t); i++ {
		diff ^= t[i]
	}
	return diff
}
