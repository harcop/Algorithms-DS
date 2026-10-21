package leetcode

import "testing"

func TestTrieExample(t *testing.T) {
	trie := NewTrie()
	trie.Insert("apple")
	if !trie.Search("apple") {
		t.Fatal("Search(apple) = false, want true")
	}
	if trie.Search("app") {
		t.Fatal("Search(app) = true, want false")
	}
	if !trie.StartsWith("app") {
		t.Fatal("StartsWith(app) = false, want true")
	}
	trie.Insert("app")
	if !trie.Search("app") {
		t.Fatal("Search(app) = false, want true")
	}
}
