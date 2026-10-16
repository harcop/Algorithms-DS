package leetcode

// LeetCode #87 - Scramble String
//
// A scramble is two scrambled halves in the same order, or swapped. Memoize each slice pair.
func isScramble(s1 string, s2 string) bool {
	if len(s1) != len(s2) {
		return false
	}
	memo := make(map[[3]int]bool)
	seen := make(map[[3]int]bool)

	var dfs func(i, j, length int) bool
	dfs = func(i, j, length int) bool {
		key := [3]int{i, j, length}
		if seen[key] {
			return memo[key]
		}
		seen[key] = true
		if s1[i:i+length] == s2[j:j+length] {
			memo[key] = true
			return true
		}

		var count [26]int
		for k := 0; k < length; k++ {
			count[s1[i+k]-'a']++
			count[s2[j+k]-'a']--
		}
		for _, c := range count {
			if c != 0 {
				return false
			}
		}

		for k := 1; k < length; k++ {
			if dfs(i, j, k) && dfs(i+k, j+k, length-k) {
				memo[key] = true
				return true
			}
			if dfs(i, j+length-k, k) && dfs(i+k, j, length-k) {
				memo[key] = true
				return true
			}
		}
		return false
	}

	return dfs(0, 0, len(s1))
}
