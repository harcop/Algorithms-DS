package leetcode

import "testing"

func TestReverseBitsExampleOne(t *testing.T) {
	if got := reverseBits(43261596); got != 964176192 {
		t.Fatalf("reverseBits() = %d, want 964176192", got)
	}
}

func TestReverseBitsExampleTwo(t *testing.T) {
	if got := reverseBits(4294967293); got != 3221225471 {
		t.Fatalf("reverseBits() = %d, want 3221225471", got)
	}
}
