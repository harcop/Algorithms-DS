package leetcode

import "testing"

func TestLexicalOrderExample(t *testing.T) {
	got := lexicalOrder(13)
	want := []int{1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9}
	if !sameInts(got, want) {
		t.Fatalf("lexicalOrder() = %v, want %v", got, want)
	}
}
