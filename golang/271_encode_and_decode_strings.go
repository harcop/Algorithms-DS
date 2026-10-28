package leetcode

import "strconv"

// LeetCode #271 - Encode and Decode Strings
//
// Prefix each string with its length so the decoder can split them apart.
func encode(strs []string) string {
	var out string
	for _, s := range strs {
		out += strconv.Itoa(len(s)) + ":" + s
	}
	return out
}

func decode(s string) []string {
	var out []string
	for i := 0; i < len(s); {
		j := i
		for j < len(s) && s[j] != ':' {
			j++
		}
		n, _ := strconv.Atoi(s[i:j])
		j++
		out = append(out, s[j:j+n])
		i = j + n
	}
	return out
}
