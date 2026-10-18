package leetcode

// LeetCode #131 - Palindrome Partitioning
//
// LeetCode names this partition; that name already belongs to list partitioning (#86).
// Precompute palindromic slices, then build every way to cut the string into them.
func partitionPalindrome(s string) [][]string {
	n := len(s)
	pal := make([][]bool, n)
	for i := range pal {
		pal[i] = make([]bool, n)
	}
	for i := n - 1; i >= 0; i-- {
		for j := i; j < n; j++ {
			pal[i][j] = s[i] == s[j] && (j-i < 2 || pal[i+1][j-1])
		}
	}

	var result [][]string
	var path []string
	var dfs func(start int)
	dfs = func(start int) {
		if start == n {
			result = append(result, append([]string(nil), path...))
			return
		}
		for end := start; end < n; end++ {
			if !pal[start][end] {
				continue
			}
			path = append(path, s[start:end+1])
			dfs(end + 1)
			path = path[:len(path)-1]
		}
	}
	dfs(0)
	return result
}
