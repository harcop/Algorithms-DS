package leetcode

import "testing"

func TestWordPatternMatchExampleOne(t *testing.T) {
	if !wordPatternMatch("abab", "redblueredblue") {
		t.Fatal("wordPatternMatch() = false, want true")
	}
}

func TestWordPatternMatchExampleTwo(t *testing.T) {
	if wordPatternMatch("ab", "aa") {
		t.Fatal("wordPatternMatch() = true, want false")
	}
}
