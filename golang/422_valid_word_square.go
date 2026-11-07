package leetcode

// LeetCode #422 - Valid Word Square
//
// Each character must equal the one across the diagonal, including words that end early.
func validWordSquare(words []string) bool {
	n := len(words)
	for i := 0; i < n; i++ {
		for j := 0; j < len(words[i]); j++ {
			if j >= n || i >= len(words[j]) || words[i][j] != words[j][i] {
				return false
			}
		}
	}
	return true
}
