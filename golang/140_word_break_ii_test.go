package leetcode

import "testing"

func TestWordBreakIIExampleOne(t *testing.T) {
	got := wordBreakII("catsanddog", []string{"cat", "cats", "and", "sand", "dog"})
	want := []string{"cats and dog", "cat sand dog"}
	if !sameStrings(got, want) {
		t.Fatalf("wordBreakII() = %v, want %v", got, want)
	}
}

func TestWordBreakIIExampleTwo(t *testing.T) {
	got := wordBreakII("pineapplepenapple", []string{"apple", "pen", "applepen", "pine", "pineapple"})
	if len(got) != 3 {
		t.Fatalf("wordBreakII() = %v, want 3 sentences", got)
	}
}
