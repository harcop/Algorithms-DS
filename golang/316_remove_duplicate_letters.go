package leetcode

// LeetCode #316 - Remove Duplicate Letters
//
// Build the smallest subsequence that still contains every letter.
func removeDuplicateLetters(s string) string {
	var freq [26]int
	for i := 0; i < len(s); i++ {
		freq[s[i]-'a']++
	}
	var used [26]bool
	stack := make([]byte, 0, 26)
	for i := 0; i < len(s); i++ {
		letter := s[i]
		index := letter - 'a'
		freq[index]--
		if used[index] {
			continue
		}
		for len(stack) > 0 && letter < stack[len(stack)-1] && freq[stack[len(stack)-1]-'a'] > 0 {
			used[stack[len(stack)-1]-'a'] = false
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, letter)
		used[index] = true
	}
	return string(stack)
}
