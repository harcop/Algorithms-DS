package leetcode

import "sort"

// LeetCode #358 - Rearrange String k Distance Apart
//
// Always place the most common letter that is outside the last k-1 positions.
func rearrangeString(s string, k int) string {
	if k <= 1 {
		return s
	}
	var count [26]int
	for i := 0; i < len(s); i++ {
		count[s[i]-'a']++
	}
	type letter struct {
		ch byte
		n  int
	}
	letters := make([]letter, 0, 26)
	for i := 0; i < 26; i++ {
		if count[i] > 0 {
			letters = append(letters, letter{byte('a' + i), count[i]})
		}
	}
	out := make([]byte, 0, len(s))
	for len(out) < len(s) {
		sort.Slice(letters, func(i, j int) bool {
			if letters[i].n != letters[j].n {
				return letters[i].n > letters[j].n
			}
			return letters[i].ch < letters[j].ch
		})
		placed := false
		window := len(out) - (k - 1)
		if window < 0 {
			window = 0
		}
		for i := range letters {
			if letters[i].n == 0 {
				break
			}
			blocked := false
			for j := window; j < len(out); j++ {
				if out[j] == letters[i].ch {
					blocked = true
					break
				}
			}
			if blocked {
				continue
			}
			out = append(out, letters[i].ch)
			letters[i].n--
			placed = true
			break
		}
		if !placed {
			return ""
		}
	}
	return string(out)
}
