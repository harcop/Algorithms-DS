package leetcode

import "testing"

func TestNumDecodingsExampleOne(t *testing.T) {
	got := numDecodings("12")
	if got != 2 {
		t.Fatalf("numDecodings() = %d, want 2", got)
	}
}

func TestNumDecodingsExampleTwo(t *testing.T) {
	got := numDecodings("226")
	if got != 3 {
		t.Fatalf("numDecodings() = %d, want 3", got)
	}
}

func TestNumDecodingsExampleThree(t *testing.T) {
	got := numDecodings("06")
	if got != 0 {
		t.Fatalf("numDecodings() = %d, want 0", got)
	}
}
