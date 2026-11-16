package leetcode

// LeetCode #557 - Reverse Words in a String III
//
// LeetCode names this reverseWords; that name already belongs to reversing word order (#151).
// Each word is reversed in place, and the spaces stay where they are.
func reverseWordsIII(s string) string {
	chars := []byte(s)
	start := 0
	for i := 0; i <= len(chars); i++ {
		if i != len(chars) && chars[i] != ' ' {
			continue
		}
		for left, right := start, i-1; left < right; left, right = left+1, right-1 {
			chars[left], chars[right] = chars[right], chars[left]
		}
		start = i + 1
	}
	return string(chars)
}
