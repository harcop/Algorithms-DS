package leetcode

import "testing"

func TestWordDictionaryExample(t *testing.T) {
	dict := NewWordDictionary()
	dict.AddWord("bad")
	dict.AddWord("dad")
	dict.AddWord("mad")
	if dict.Search("pad") {
		t.Fatal("Search(pad) = true, want false")
	}
	if !dict.Search("bad") {
		t.Fatal("Search(bad) = false, want true")
	}
	if !dict.Search(".ad") {
		t.Fatal("Search(.ad) = false, want true")
	}
	if !dict.Search("b..") {
		t.Fatal("Search(b..) = false, want true")
	}
}
