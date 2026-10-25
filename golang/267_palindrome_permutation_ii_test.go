package leetcode

import "testing"

func TestGeneratePalindromesExampleOne(t *testing.T) {
	got := generatePalindromes("aabb")
	want := []string{"abba", "baab"}
	if !sameStrings(got, want) {
		t.Fatalf("generatePalindromes() = %v, want %v", got, want)
	}
}

func TestGeneratePalindromesExampleTwo(t *testing.T) {
	if len(generatePalindromes("abc")) != 0 {
		t.Fatal("generatePalindromes() returned palindromes, want none")
	}
}
