package leetcode

import "testing"

func TestReverseStringExampleOne(t *testing.T) {
	s := []byte("hello")
	reverseString(s)
	if string(s) != "olleh" {
		t.Fatalf("reverseString() = %q, want %q", s, "olleh")
	}
}
