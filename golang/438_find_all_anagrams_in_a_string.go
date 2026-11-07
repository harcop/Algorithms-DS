package leetcode

// LeetCode #438 - Find All Anagrams in a String
//
// A window the length of the pattern matches when both letter counts are equal.
func findAnagrams(s, p string) []int {
	if len(p) > len(s) {
		return nil
	}
	var need, window [26]int
	for i := 0; i < len(p); i++ {
		need[p[i]-'a']++
		window[s[i]-'a']++
	}
	var out []int
	if window == need {
		out = append(out, 0)
	}
	for i := len(p); i < len(s); i++ {
		window[s[i]-'a']++
		window[s[i-len(p)]-'a']--
		if window == need {
			out = append(out, i-len(p)+1)
		}
	}
	return out
}
