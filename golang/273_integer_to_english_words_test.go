package leetcode

import "testing"

func TestNumberToWordsExampleOne(t *testing.T) {
	got := numberToWords(123)
	want := "One Hundred Twenty Three"
	if got != want {
		t.Fatalf("numberToWords() = %q, want %q", got, want)
	}
}

func TestNumberToWordsExampleTwo(t *testing.T) {
	got := numberToWords(12345)
	want := "Twelve Thousand Three Hundred Forty Five"
	if got != want {
		t.Fatalf("numberToWords() = %q, want %q", got, want)
	}
}
