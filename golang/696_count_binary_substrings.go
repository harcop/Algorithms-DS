package leetcode

// LeetCode #696 - Count Binary Substrings
//
// Adjacent runs of zeros and ones form min(left, right) balanced groups where they meet.
func countBinarySubstrings(s string) int {
	prev, cur, total := 0, 1, 0
	for i := 1; i < len(s); i++ {
		if s[i] == s[i-1] {
			cur++
			continue
		}
		total += min(prev, cur)
		prev, cur = cur, 1
	}
	return total + min(prev, cur)
}
