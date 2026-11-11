package leetcode

import "strings"

// LeetCode #482 - License Key Formatting
//
// Drop the dashes, uppercase the rest, and insert a dash before every group of k characters from the right.
func licenseKeyFormatting(s string, k int) string {
	raw := make([]byte, 0, len(s))
	for i := 0; i < len(s); i++ {
		if s[i] == '-' {
			continue
		}
		c := s[i]
		if c >= 'a' && c <= 'z' {
			c -= 'a' - 'A'
		}
		raw = append(raw, c)
	}
	if len(raw) == 0 {
		return ""
	}
	var out strings.Builder
	first := len(raw) % k
	if first == 0 {
		first = k
	}
	out.Write(raw[:first])
	for i := first; i < len(raw); i += k {
		out.WriteByte('-')
		out.Write(raw[i : i+k])
	}
	return out.String()
}
