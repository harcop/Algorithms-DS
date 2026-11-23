package leetcode

import "testing"

func TestReplaceWordsExample(t *testing.T) {
	got := replaceWords([]string{"cat", "bat", "rat"}, "the cattle was rattled by the battery")
	want := "the cat was rat by the bat"
	if got != want {
		t.Fatalf("replaceWords() = %q, want %q", got, want)
	}
}
