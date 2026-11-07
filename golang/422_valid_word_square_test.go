package leetcode

import "testing"

func TestValidWordSquareExamples(t *testing.T) {
	if !validWordSquare([]string{"abcd", "bnrt", "crmy", "dtye"}) {
		t.Fatal("validWordSquare(full square) = false, want true")
	}
	if !validWordSquare([]string{"abcd", "bnrt", "crm", "dt"}) {
		t.Fatal("validWordSquare(ragged square) = false, want true")
	}
	if validWordSquare([]string{"ball", "asee", "let", "lep"}) {
		t.Fatal("validWordSquare(ragged mismatch) = true, want false")
	}
	if validWordSquare([]string{"abc", "b"}) {
		t.Fatal("validWordSquare(short column) = true, want false")
	}
}
