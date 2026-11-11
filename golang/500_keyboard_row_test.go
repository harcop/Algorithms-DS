package leetcode

import "testing"

func TestFindKeyboardWordsExample(t *testing.T) {
	got := findKeyboardWords([]string{"Hello", "Alaska", "Dad", "Peace"})
	want := []string{"Alaska", "Dad"}
	if len(got) != len(want) {
		t.Fatalf("findKeyboardWords() = %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("findKeyboardWords() = %v, want %v", got, want)
		}
	}
}
