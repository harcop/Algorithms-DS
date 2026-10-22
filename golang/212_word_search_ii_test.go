package leetcode

import "testing"

func TestFindWordsExampleOne(t *testing.T) {
	board := [][]byte{
		[]byte("oaan"),
		[]byte("etae"),
		[]byte("ihkr"),
		[]byte("iflv"),
	}
	got := findWords(board, []string{"oath", "pea", "eat", "rain"})
	want := []string{"eat", "oath"}
	if !sameStrings(got, want) {
		t.Fatalf("findWords() = %v, want %v", got, want)
	}
}
