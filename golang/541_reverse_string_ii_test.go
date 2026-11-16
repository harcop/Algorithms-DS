package leetcode

import "testing"

func TestReverseStrExample(t *testing.T) {
	if got := reverseStr("abcdefg", 2); got != "bacdfeg" {
		t.Fatalf("reverseStr(abcdefg, 2) = %q, want bacdfeg", got)
	}
}
