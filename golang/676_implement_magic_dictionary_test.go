package leetcode

import "testing"

func TestMagicDictionaryExample(t *testing.T) {
	dict := NewMagicDictionary()
	dict.BuildDict([]string{"hello", "leetcode"})
	if dict.Search("hello") {
		t.Fatal("Search(hello) = true, want false")
	}
	if !dict.Search("hhllo") {
		t.Fatal("Search(hhllo) = false, want true")
	}
	if dict.Search("hell") {
		t.Fatal("Search(hell) = true, want false")
	}
	if dict.Search("leetcoded") {
		t.Fatal("Search(leetcoded) = true, want false")
	}
}
