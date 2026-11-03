package leetcode

import "testing"

func TestLengthLongestPathExample(t *testing.T) {
	input := "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
	if got := lengthLongestPath(input); got != 32 {
		t.Fatalf("lengthLongestPath() = %d, want 32", got)
	}
	if got := lengthLongestPath("dir\n\tsubdir1"); got != 0 {
		t.Fatalf("lengthLongestPath() = %d, want 0", got)
	}
}
