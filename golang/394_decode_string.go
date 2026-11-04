package leetcode

import "strings"

// LeetCode #394 - Decode String
//
// A repeat count applies to the string built inside the matching brackets.
func decodeString(s string) string {
	var counts []int
	var parts []string
	cur := ""
	k := 0
	for i := 0; i < len(s); i++ {
		ch := s[i]
		switch {
		case ch >= '0' && ch <= '9':
			k = k*10 + int(ch-'0')
		case ch == '[':
			counts = append(counts, k)
			parts = append(parts, cur)
			cur = ""
			k = 0
		case ch == ']':
			times := counts[len(counts)-1]
			counts = counts[:len(counts)-1]
			prev := parts[len(parts)-1]
			parts = parts[:len(parts)-1]
			cur = prev + strings.Repeat(cur, times)
		default:
			cur += string(ch)
		}
	}
	return cur
}
