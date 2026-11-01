package leetcode

import "strconv"

// LeetCode #320 - Generalized Abbreviation
//
// Each bit mask decides which letters stay and which runs of letters become a count.
func generateAbbreviations(word string) []string {
	n := len(word)
	out := make([]string, 0, 1<<n)
	for mask := 0; mask < 1<<n; mask++ {
		var text string
		for i := 0; i < n; {
			if mask>>i&1 == 0 {
				text += word[i : i+1]
				i++
				continue
			}
			j := i
			for j < n && mask>>j&1 == 1 {
				j++
			}
			text += strconv.Itoa(j - i)
			i = j
		}
		out = append(out, text)
	}
	return out
}
