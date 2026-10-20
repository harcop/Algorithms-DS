package leetcode

// LeetCode #186 - Reverse Words in a String II
//
// LeetCode names this reverseWords; that name already belongs to the allocating version (#151).
// Reverse the whole buffer, then reverse each word back into place.
func reverseWordsII(s []byte) {
	n := len(s)
	if n == 0 {
		return
	}
	reverse := func(i, j int) {
		for i < j {
			s[i], s[j] = s[j], s[i]
			i++
			j--
		}
	}
	reverse(0, n-1)
	for i := 0; i < n; {
		j := i
		for j < n && s[j] != ' ' {
			j++
		}
		reverse(i, j-1)
		i = j + 1
	}
}
