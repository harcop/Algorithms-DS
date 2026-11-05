package leetcode

// LeetCode #408 - Valid Word Abbreviation
//
// A number skips that many letters, and a leading zero is not a valid skip.
func validWordAbbreviation(word string, abbr string) bool {
	i, j := 0, 0
	for i < len(word) && j < len(abbr) {
		if abbr[j] >= '0' && abbr[j] <= '9' {
			if abbr[j] == '0' {
				return false
			}
			skip := 0
			for j < len(abbr) && abbr[j] >= '0' && abbr[j] <= '9' {
				skip = skip*10 + int(abbr[j]-'0')
				j++
			}
			i += skip
			continue
		}
		if word[i] != abbr[j] {
			return false
		}
		i++
		j++
	}
	return i == len(word) && j == len(abbr)
}
