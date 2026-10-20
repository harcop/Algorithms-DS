package leetcode

import "sort"

// LeetCode #187 - Repeated DNA Sequences
//
// A 10-letter window that appears twice is a repeated sequence.
func findRepeatedDnaSequences(s string) []string {
	if len(s) < 10 {
		return nil
	}
	seen := map[string]struct{}{}
	twice := map[string]struct{}{}
	for i := 0; i <= len(s)-10; i++ {
		key := s[i : i+10]
		if _, ok := seen[key]; ok {
			twice[key] = struct{}{}
		} else {
			seen[key] = struct{}{}
		}
	}
	out := make([]string, 0, len(twice))
	for key := range twice {
		out = append(out, key)
	}
	sort.Strings(out)
	return out
}
