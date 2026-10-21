package leetcode

// LeetCode #208 - Implement Trie (Prefix Tree)
//
// Each letter is an edge, and a mark records where a full word ends.
type Trie struct {
	root *trieNode
}

type trieNode struct {
	children [26]*trieNode
	end      bool
}

func NewTrie() Trie {
	return Trie{root: &trieNode{}}
}

func (trie *Trie) Insert(word string) {
	node := trie.root
	for i := 0; i < len(word); i++ {
		index := word[i] - 'a'
		if node.children[index] == nil {
			node.children[index] = &trieNode{}
		}
		node = node.children[index]
	}
	node.end = true
}

func (trie *Trie) Search(word string) bool {
	node := trie.walk(word)
	return node != nil && node.end
}

func (trie *Trie) StartsWith(prefix string) bool {
	return trie.walk(prefix) != nil
}

func (trie *Trie) walk(s string) *trieNode {
	node := trie.root
	for i := 0; i < len(s); i++ {
		node = node.children[s[i]-'a']
		if node == nil {
			return nil
		}
	}
	return node
}
