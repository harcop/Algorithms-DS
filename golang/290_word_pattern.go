package leetcode

import "strings"

// LeetCode #290 - Word Pattern
//
// Each pattern letter maps to one word, and each word maps back to one letter.
func wordPattern(pattern string, s string) bool {
	words := strings.Fields(s)
	if len(pattern) != len(words) {
		return false
	}
	toWord := map[byte]string{}
	toLetter := map[string]byte{}
	for i := 0; i < len(pattern); i++ {
		letter, word := pattern[i], words[i]
		if existing, ok := toWord[letter]; ok && existing != word {
			return false
		}
		if existing, ok := toLetter[word]; ok && existing != letter {
			return false
		}
		toWord[letter] = word
		toLetter[word] = letter
	}
	return true
}
