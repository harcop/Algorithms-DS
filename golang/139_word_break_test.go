package leetcode

import "testing"

func TestWordBreakExampleOne(t *testing.T) {
	if !wordBreak("leetcode", []string{"leet", "code"}) {
		t.Fatal("wordBreak() = false, want true")
	}
}

func TestWordBreakExampleTwo(t *testing.T) {
	if !wordBreak("applepenapple", []string{"apple", "pen"}) {
		t.Fatal("wordBreak() = false, want true")
	}
}

func TestWordBreakExampleThree(t *testing.T) {
	if wordBreak("catsandog", []string{"cats", "dog", "sand", "and", "cat"}) {
		t.Fatal("wordBreak() = true, want false")
	}
}
