package leetcode

import "strconv"

// LeetCode #288 - Unique Word Abbreviation
//
// An abbreviation is unique when no other dictionary word shortens to the same form.
type ValidWordAbbr struct {
	words map[string][]string
}

func NewValidWordAbbr(dictionary []string) ValidWordAbbr {
	words := map[string][]string{}
	for _, word := range dictionary {
		key := abbreviate(word)
		words[key] = append(words[key], word)
	}
	return ValidWordAbbr{words: words}
}

func (dict ValidWordAbbr) IsUnique(word string) bool {
	matches := dict.words[abbreviate(word)]
	return len(matches) == 0 || (len(matches) == 1 && matches[0] == word)
}

func abbreviate(word string) string {
	if len(word) <= 2 {
		return word
	}
	return string(word[0]) + strconv.Itoa(len(word)-2) + string(word[len(word)-1])
}
