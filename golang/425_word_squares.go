package leetcode

// LeetCode #425 - Word Squares
//
// Each next word is taken from those that match the prefix already written down the column.
func wordSquares(words []string) [][]string {
	if len(words) == 0 {
		return nil
	}
	n := len(words[0])
	byPrefix := map[string][]string{}
	for _, word := range words {
		for i := 1; i < len(word); i++ {
			byPrefix[word[:i]] = append(byPrefix[word[:i]], word)
		}
	}

	var squares [][]string
	cur := make([]string, 0, n)
	var search func()
	search = func() {
		if len(cur) == n {
			squares = append(squares, append([]string(nil), cur...))
			return
		}
		row := len(cur)
		prefix := make([]byte, row)
		for i := 0; i < row; i++ {
			prefix[i] = cur[i][row]
		}
		for _, word := range byPrefix[string(prefix)] {
			cur = append(cur, word)
			search()
			cur = cur[:len(cur)-1]
		}
	}
	for _, word := range words {
		cur = append(cur, word)
		search()
		cur = cur[:0]
	}
	return squares
}
