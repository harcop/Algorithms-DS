package leetcode

import "sort"

// LeetCode #212 - Word Search II
//
// Put the words in a trie and walk the board only along letters that trie still contains.
func findWords(board [][]byte, words []string) []string {
	if len(board) == 0 || len(board[0]) == 0 {
		return nil
	}
	root := &boardWordNode{}
	for _, word := range words {
		node := root
		for i := 0; i < len(word); i++ {
			index := word[i] - 'a'
			if node.children[index] == nil {
				node.children[index] = &boardWordNode{}
			}
			node = node.children[index]
		}
		node.word = word
	}
	m, n := len(board), len(board[0])
	found := map[string]struct{}{}
	var walk func(node *boardWordNode, i, j int)
	walk = func(node *boardWordNode, i, j int) {
		if i < 0 || j < 0 || i >= m || j >= n || board[i][j] == '#' {
			return
		}
		ch := board[i][j]
		child := node.children[ch-'a']
		if child == nil {
			return
		}
		if child.word != "" {
			found[child.word] = struct{}{}
			child.word = ""
		}
		board[i][j] = '#'
		walk(child, i-1, j)
		walk(child, i+1, j)
		walk(child, i, j-1)
		walk(child, i, j+1)
		board[i][j] = ch
	}
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			walk(root, i, j)
		}
	}
	out := make([]string, 0, len(found))
	for word := range found {
		out = append(out, word)
	}
	sort.Strings(out)
	return out
}

type boardWordNode struct {
	children [26]*boardWordNode
	word     string
}
