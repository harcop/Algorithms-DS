package leetcode

// LeetCode #291 - Word Pattern II
//
// Assign each pattern letter a substring, and reject a letter that reuses another letter's text.
func wordPatternMatch(pattern string, s string) bool {
	toText := map[byte]string{}
	used := map[string]byte{}
	var match func(pi, si int) bool
	match = func(pi, si int) bool {
		if pi == len(pattern) && si == len(s) {
			return true
		}
		if pi == len(pattern) || si == len(s) {
			return false
		}
		letter := pattern[pi]
		if text, ok := toText[letter]; ok {
			if si+len(text) <= len(s) && s[si:si+len(text)] == text {
				return match(pi+1, si+len(text))
			}
			return false
		}
		for end := si + 1; end <= len(s); end++ {
			text := s[si:end]
			if _, taken := used[text]; taken {
				continue
			}
			toText[letter] = text
			used[text] = letter
			if match(pi+1, end) {
				return true
			}
			delete(used, text)
			delete(toText, letter)
		}
		return false
	}
	return match(0, 0)
}
