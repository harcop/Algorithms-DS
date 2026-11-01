package leetcode

import "testing"

func TestMaxProductWordsExample(t *testing.T) {
	words := []string{"abcw", "baz", "foo", "bar", "xtfn", "abcdef"}
	if got := maxProductWords(words); got != 16 {
		t.Fatalf("maxProductWords() = %d, want 16", got)
	}
}
