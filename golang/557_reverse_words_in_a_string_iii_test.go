package leetcode

import "testing"

func TestReverseWordsIIIExample(t *testing.T) {
	got := reverseWordsIII("Let's take LeetCode contest")
	if got != "s'teL ekat edoCteeL tsetnoc" {
		t.Fatalf("reverseWordsIII = %q, want s'teL ekat edoCteeL tsetnoc", got)
	}
}
