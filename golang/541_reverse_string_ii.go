package leetcode

// LeetCode #541 - Reverse String II
//
// Reverse every other block of k characters, and reverse a short tail when the string runs out.
func reverseStr(s string, k int) string {
	chars := []byte(s)
	for i := 0; i < len(chars); i += 2 * k {
		end := i + k
		if end > len(chars) {
			end = len(chars)
		}
		for left, right := i, end-1; left < right; left, right = left+1, right-1 {
			chars[left], chars[right] = chars[right], chars[left]
		}
	}
	return string(chars)
}
