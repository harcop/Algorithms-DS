package leetcode

import "testing"

func TestFindRepeatedDnaSequencesExampleOne(t *testing.T) {
	got := findRepeatedDnaSequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT")
	want := []string{"AAAAACCCCC", "CCCCCAAAAA"}
	if !sameStrings(got, want) {
		t.Fatalf("findRepeatedDnaSequences() = %v, want %v", got, want)
	}
}

func TestFindRepeatedDnaSequencesExampleTwo(t *testing.T) {
	got := findRepeatedDnaSequences("AAAAAAAAAAAAA")
	want := []string{"AAAAAAAAAA"}
	if !sameStrings(got, want) {
		t.Fatalf("findRepeatedDnaSequences() = %v, want %v", got, want)
	}
}
