package leetcode

import "testing"

func TestRead4IIStaggeredReads(t *testing.T) {
	reader := NewRead4II("abcdef")
	a := make([]byte, 8)
	b := make([]byte, 8)
	if got := reader.Read(a, 1); got != 1 || string(a[:1]) != "a" {
		t.Fatalf("Read() = %d %q, want 1 %q", got, a[:1], "a")
	}
	if got := reader.Read(b, 4); got != 4 || string(b[:4]) != "bcde" {
		t.Fatalf("Read() = %d %q, want 4 %q", got, b[:4], "bcde")
	}
	if got := reader.Read(a, 10); got != 1 || string(a[:1]) != "f" {
		t.Fatalf("Read() = %d %q, want 1 %q", got, a[:1], "f")
	}
}
