package leetcode

import "strconv"

// LeetCode #38 - Count and Say
//
// Start from "1" and repeatedly replace each run of identical digits with its length and digit.
func countAndSay(n int) string {
	if n <= 0 {
		return ""
	}

	s := "1"
	for step := 1; step < n; step++ {
		s = nextCountAndSay(s)
	}
	return s
}

func nextCountAndSay(s string) string {
	var out []byte
	for i := 0; i < len(s); {
		j := i
		for j < len(s) && s[j] == s[i] {
			j++
		}
		out = append(out, strconv.Itoa(j-i)...)
		out = append(out, s[i])
		i = j
	}
	return string(out)
}
