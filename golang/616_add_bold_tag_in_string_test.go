package leetcode

import "testing"

func TestAddBoldTagExamples(t *testing.T) {
	if got := addBoldTag("abcxyz123", []string{"abc", "123"}); got != "<b>abc</b>xyz<b>123</b>" {
		t.Fatalf("addBoldTag() = %q, want <b>abc</b>xyz<b>123</b>", got)
	}
	if got := addBoldTag("aaabbb", []string{"aa", "b"}); got != "<b>aaabbb</b>" {
		t.Fatalf("addBoldTag() = %q, want <b>aaabbb</b>", got)
	}
}
