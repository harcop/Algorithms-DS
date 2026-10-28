package leetcode

import "testing"

func TestWordPatternExampleOne(t *testing.T) {
	if !wordPattern("abba", "dog cat cat dog") {
		t.Fatal("wordPattern() = false, want true")
	}
}

func TestWordPatternExampleTwo(t *testing.T) {
	if wordPattern("abba", "dog cat cat fish") {
		t.Fatal("wordPattern() = true, want false")
	}
}
