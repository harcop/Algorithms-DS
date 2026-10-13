package leetcode

import "testing"

func TestStrStrExampleOne(t *testing.T) {
	if got := strStr("sadbutsad", "sad"); got != 0 {
		t.Fatalf("strStr() = %d, want 0", got)
	}
}

func TestStrStrExampleTwo(t *testing.T) {
	if got := strStr("leetcode", "leeto"); got != -1 {
		t.Fatalf("strStr() = %d, want -1", got)
	}
}
