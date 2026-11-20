package leetcode

import "strings"

// LeetCode #616 - Add Bold Tag in String
//
// Mark every character covered by a word, then wrap each run of marks with one bold pair.
func addBoldTag(s string, words []string) string {
	bold := make([]bool, len(s))
	for _, word := range words {
		if word == "" {
			continue
		}
		for i := 0; i+len(word) <= len(s); i++ {
			if s[i:i+len(word)] != word {
				continue
			}
			for j := i; j < i+len(word); j++ {
				bold[j] = true
			}
		}
	}
	var b strings.Builder
	for i := 0; i < len(s); i++ {
		if bold[i] && (i == 0 || !bold[i-1]) {
			b.WriteString("<b>")
		}
		b.WriteByte(s[i])
		if bold[i] && (i == len(s)-1 || !bold[i+1]) {
			b.WriteString("</b>")
		}
	}
	return b.String()
}
