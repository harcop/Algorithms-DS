package leetcode

import "testing"

func TestCompressExample(t *testing.T) {
	chars := []byte{'a', 'a', 'b', 'b', 'c', 'c', 'c'}
	got := compress(chars)
	if got != 6 {
		t.Fatalf("compress() = %d, want 6", got)
	}
	want := []byte{'a', '2', 'b', '2', 'c', '3'}
	if !sameBytes(chars[:got], want) {
		t.Fatalf("compress() wrote %q, want %q", chars[:got], want)
	}
}
