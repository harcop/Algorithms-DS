package leetcode

import "testing"

func TestRead4AcrossChunks(t *testing.T) {
	reader := NewRead4("abcdef")
	buf := make([]byte, 10)
	if got := reader.Read(buf, 5); got != 5 || string(buf[:5]) != "abcde" {
		t.Fatalf("Read() = %d %q, want 5 %q", got, buf[:5], "abcde")
	}
}

func TestRead4PartialLastChunk(t *testing.T) {
	reader := NewRead4("abcd")
	buf := make([]byte, 10)
	if got := reader.Read(buf, 3); got != 3 || string(buf[:3]) != "abc" {
		t.Fatalf("Read() = %d %q, want 3 %q", got, buf[:3], "abc")
	}
}

func TestRead4MultipleReads(t *testing.T) {
	reader := NewRead4("abcdefgh")
	buf := make([]byte, 10)
	if got := reader.Read(buf, 3); got != 3 || string(buf[:3]) != "abc" {
		t.Fatalf("Read() = %d %q, want 3 %q", got, buf[:3], "abc")
	}
	if got := reader.Read(buf, 4); got != 4 || string(buf[:4]) != "defg" {
		t.Fatalf("Read() = %d %q, want 4 %q", got, buf[:4], "defg")
	}
}
