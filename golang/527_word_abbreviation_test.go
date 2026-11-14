package leetcode

import "testing"

func TestWordsAbbreviationExample(t *testing.T) {
	words := []string{"like", "god", "internal", "me", "internet", "interval", "intension", "face", "intrusion"}
	got := wordsAbbreviation(words)
	want := []string{"l2e", "god", "internal", "me", "i6t", "interval", "inte4n", "f2e", "intr4n"}
	if len(got) != len(want) {
		t.Fatalf("wordsAbbreviation() = %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("wordsAbbreviation() = %v, want %v", got, want)
		}
	}
}
