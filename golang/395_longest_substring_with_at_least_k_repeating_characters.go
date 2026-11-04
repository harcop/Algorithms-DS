package leetcode

// LeetCode #395 - Longest Substring with At Least K Repeating Characters
//
// A character seen too rarely cannot sit inside the answer, so the search splits around it.
func longestSubstring(s string, k int) int {
	var search func(string) int
	search = func(part string) int {
		if part == "" || k == 0 {
			return 0
		}
		var count [26]int
		for i := 0; i < len(part); i++ {
			count[part[i]-'a']++
		}
		for i := 0; i < len(part); i++ {
			if count[part[i]-'a'] < k {
				left := search(part[:i])
				right := search(part[i+1:])
				if left > right {
					return left
				}
				return right
			}
		}
		return len(part)
	}
	return search(s)
}
