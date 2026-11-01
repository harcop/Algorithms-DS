package leetcode

import "testing"

func TestReverseVowelsExampleOne(t *testing.T) {
	if got := reverseVowels("hello"); got != "holle" {
		t.Fatalf("reverseVowels() = %q, want %q", got, "holle")
	}
}
