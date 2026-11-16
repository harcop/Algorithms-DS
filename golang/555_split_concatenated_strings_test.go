package leetcode

import "testing"

func TestSplitLoopedStringExample(t *testing.T) {
	got := splitLoopedString([]string{"abc", "xyz"})
	if got != "zyxcba" {
		t.Fatalf("splitLoopedString = %q, want zyxcba", got)
	}
}
