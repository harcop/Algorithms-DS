package leetcode

// LeetCode #211 - Design Add and Search Words Data Structure
//
// Store words in a trie, and let '.' match any one letter.
type WordDictionary struct {
	root *wordDictNode
}

type wordDictNode struct {
	children [26]*wordDictNode
	end      bool
}

func NewWordDictionary() WordDictionary {
	return WordDictionary{root: &wordDictNode{}}
}

func (dict *WordDictionary) AddWord(word string) {
	node := dict.root
	for i := 0; i < len(word); i++ {
		index := word[i] - 'a'
		if node.children[index] == nil {
			node.children[index] = &wordDictNode{}
		}
		node = node.children[index]
	}
	node.end = true
}

func (dict *WordDictionary) Search(word string) bool {
	var match func(node *wordDictNode, i int) bool
	match = func(node *wordDictNode, i int) bool {
		if i == len(word) {
			return node.end
		}
		if word[i] == '.' {
			for _, child := range node.children {
				if child != nil && match(child, i+1) {
					return true
				}
			}
			return false
		}
		child := node.children[word[i]-'a']
		return child != nil && match(child, i+1)
	}
	return match(dict.root, 0)
}
