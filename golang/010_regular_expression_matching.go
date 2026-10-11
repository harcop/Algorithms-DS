package leetcode

// LeetCode #10 - Regular Expression Matching
//
// '.' matches any single character; '*' matches zero or more of the preceding element.
// Full match of the entire input string is required.
func isMatch(s string, p string) bool {
	memo := make([][]int, len(s)+1)
	for i := range memo {
		memo[i] = make([]int, len(p)+1)
	}

	var dfs func(i, j int) bool
	dfs = func(i, j int) bool {
		if memo[i][j] != 0 {
			return memo[i][j] == 1
		}

		var out bool
		if j == len(p) {
			out = i == len(s)
		} else {
			first := i < len(s) && (p[j] == s[i] || p[j] == '.')
			if j+1 < len(p) && p[j+1] == '*' {
				out = dfs(i, j+2) || (first && dfs(i+1, j))
			} else {
				out = first && dfs(i+1, j+1)
			}
		}

		if out {
			memo[i][j] = 1
		} else {
			memo[i][j] = 2
		}
		return out
	}

	return dfs(0, 0)
}
