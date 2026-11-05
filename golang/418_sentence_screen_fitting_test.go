package leetcode

import "testing"

func TestWordsTypingExamples(t *testing.T) {
	if got := wordsTyping([]string{"hello", "world"}, 2, 8); got != 1 {
		t.Fatalf("wordsTyping() = %d, want 1", got)
	}
	if got := wordsTyping([]string{"a", "bcd", "e"}, 3, 6); got != 2 {
		t.Fatalf("wordsTyping() = %d, want 2", got)
	}
	if got := wordsTyping([]string{"i", "had", "apple", "pie"}, 4, 5); got != 1 {
		t.Fatalf("wordsTyping() = %d, want 1", got)
	}
}
