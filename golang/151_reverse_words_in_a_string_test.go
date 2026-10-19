package leetcode

import "testing"

func TestReverseWordsExampleOne(t *testing.T) {
	got := reverseWords("the sky is blue")
	if got != "blue is sky the" {
		t.Fatalf("reverseWords() = %q, want %q", got, "blue is sky the")
	}
}

func TestReverseWordsExampleTwo(t *testing.T) {
	got := reverseWords("  hello world  ")
	if got != "world hello" {
		t.Fatalf("reverseWords() = %q, want %q", got, "world hello")
	}
}

func TestReverseWordsExampleThree(t *testing.T) {
	got := reverseWords("a good   example")
	if got != "example good a" {
		t.Fatalf("reverseWords() = %q, want %q", got, "example good a")
	}
}
