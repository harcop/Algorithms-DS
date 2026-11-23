package leetcode

import "strings"

// LeetCode #648 - Replace Words
//
// A trie of roots stops at the shortest root that prefixes a word.
type wordRoot struct {
	children [26]*wordRoot
	word     string
}

func replaceWords(dictionary []string, sentence string) string {
	root := &wordRoot{}
	for _, word := range dictionary {
		node := root
		for i := 0; i < len(word); i++ {
			index := word[i] - 'a'
			if node.children[index] == nil {
				node.children[index] = &wordRoot{}
			}
			node = node.children[index]
		}
		node.word = word
	}
	parts := strings.Split(sentence, " ")
	for i, word := range parts {
		node := root
		for j := 0; j < len(word); j++ {
			index := word[j] - 'a'
			if node.children[index] == nil {
				break
			}
			node = node.children[index]
			if node.word != "" {
				parts[i] = node.word
				break
			}
		}
	}
	return strings.Join(parts, " ")
}
